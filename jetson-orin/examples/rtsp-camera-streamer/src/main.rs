use anyhow::{Context, Result};
use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_rtsp_server as gst_rtsp_server;
use gst_rtsp_server::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about = "RTSP Camera Streamer for Jetson Orin Nano", long_about = None)]
struct Args {
    /// Camera device (e.g., /dev/video0 for USB, nvarguscamerasrc for CSI)
    #[arg(short, long, default_value = "/dev/video0")]
    device: String,

    /// RTSP server port
    #[arg(short, long, default_value = "8554")]
    port: u16,

    /// RTSP mount point (stream path)
    #[arg(short, long, default_value = "/camera")]
    mount: String,

    /// Video width
    #[arg(long, default_value = "1280")]
    width: i32,

    /// Video height
    #[arg(long, default_value = "720")]
    height: i32,

    /// Framerate
    #[arg(long, default_value = "30")]
    framerate: i32,

    /// Use CSI camera (nvarguscamerasrc) instead of USB
    #[arg(long)]
    csi: bool,
}

fn build_pipeline(args: &Args) -> String {
    if args.csi {
        // CSI camera pipeline (Jetson-specific)
        format!(
            "nvarguscamerasrc ! video/x-raw(memory:NVMM),width={},height={},framerate={}/1,format=NV12 ! \
             nvvidconv ! video/x-raw,format=I420 ! \
             x264enc tune=zerolatency bitrate=2000 speed-preset=superfast ! \
             rtph264pay name=pay0 pt=96",
            args.width, args.height, args.framerate
        )
    } else {
        // USB camera pipeline (v4l2src)
        format!(
            "v4l2src device={} ! video/x-raw,width={},height={},framerate={}/1 ! \
             videoconvert ! video/x-raw,format=I420 ! \
             x264enc tune=zerolatency bitrate=2000 speed-preset=superfast ! \
             rtph264pay name=pay0 pt=96",
            args.device, args.width, args.height, args.framerate
        )
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize GStreamer
    gst::init().context("Failed to initialize GStreamer")?;

    println!("🎥 RTSP Camera Streamer");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    if args.csi {
        println!("📹 Camera: CSI (nvarguscamerasrc)");
    } else {
        println!("📹 Camera: USB ({})", args.device);
    }
    
    println!("🔧 Resolution: {}x{} @ {}fps", args.width, args.height, args.framerate);
    println!("🌐 RTSP URL: rtsp://{}:{}{}", 
        get_local_ip().unwrap_or_else(|_| "localhost".to_string()),
        args.port, 
        args.mount
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create RTSP server
    let main_loop = glib::MainLoop::new(None, false);
    let server = gst_rtsp_server::RTSPServer::new();
    
    server.set_service(&args.port.to_string());

    // Get the mount points
    let mounts = server.mount_points().context("Failed to get mount points")?;

    // Create factory
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    
    // Build and set the pipeline
    let pipeline = build_pipeline(&args);
    println!("📋 GStreamer Pipeline:");
    println!("{}\n", pipeline);
    
    factory.set_launch(&pipeline);
    factory.set_shared(true);

    // Attach factory to mount point
    mounts.add_factory(&args.mount, factory);

    // Attach server to default main context
    let id = server.attach(None).context("Failed to attach server")?;

    println!("✅ RTSP server started!");
    println!("📺 View with VLC or ffplay:");
    println!("   vlc rtsp://{}:{}{}", 
        get_local_ip().unwrap_or_else(|_| "<jetson-ip>".to_string()),
        args.port, 
        args.mount
    );
    println!("\n⏸️  Press Ctrl+C to stop\n");

    // Run main loop
    main_loop.run();

    // Cleanup
    gst::deinit();
    glib::source_remove(id);

    Ok(())
}

fn get_local_ip() -> Result<String> {
    use std::net::UdpSocket;
    
    // Connect to Google DNS to determine local IP
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let addr = socket.local_addr()?;
    
    Ok(addr.ip().to_string())
}
