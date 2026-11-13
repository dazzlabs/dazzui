param(
  [Parameter(Mandatory=$true)][string]$NativeApiRoot,
  [Parameter(Mandatory=$true)][string]$OutputPrefix,
  [ValidateRange(4,60)][int]$Seconds = 20
)
$ErrorActionPreference = 'Stop'
$exampleRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../Example'))
$exe = Join-Path $exampleRoot '.build/x86_64-unknown-windows-msvc/release/Example.exe'
$probeExe = Join-Path $NativeApiRoot 'tools/gui/frame_probe/build/Release/frame_probe.exe'
. (Join-Path $NativeApiRoot '.agents/skills/gui-test/scripts/windows/winput.ps1')
[WInput]::Init()
Assert-Idle

Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
public static class InteractionDriver {
  [StructLayout(LayoutKind.Sequential)] struct POINT { public int x,y; }
  [StructLayout(LayoutKind.Sequential)] struct MOUSEINPUT { public int dx,dy; public uint data,flags,time; public IntPtr extra; }
  [StructLayout(LayoutKind.Sequential)] struct INPUT { public uint type; public MOUSEINPUT mouse; }
  [DllImport("user32.dll")] static extern uint SendInput(uint count, INPUT[] input, int size);
  [DllImport("user32.dll")] static extern bool GetCursorPos(out POINT point);
  [DllImport("user32.dll")] static extern short GetAsyncKeyState(int key);
  [DllImport("user32.dll")] static extern IntPtr WindowFromPoint(POINT point);
  [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr window,out uint pid);
  [DllImport("user32.dll")] static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] static extern int GetSystemMetrics(int index);
  [DllImport("kernel32.dll",CharSet=CharSet.Unicode)] static extern IntPtr CreateWaitableTimerEx(IntPtr attributes,string name,uint flags,uint access);
  [DllImport("kernel32.dll")] static extern bool SetWaitableTimer(IntPtr timer,ref long due,int period,IntPtr callback,IntPtr argument,bool resume);
  [DllImport("kernel32.dll")] static extern uint WaitForSingleObject(IntPtr handle,uint timeout);
  [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr handle);
  static uint Owner(int x,int y) { uint pid; GetWindowThreadProcessId(WindowFromPoint(new POINT{x=x,y=y}),out pid); return pid; }
  public static void Run(int processId,int left,int top,int right,int bottom,double reverseSeconds,int seconds,string path) {
    if (right<=left || bottom<=top || reverseSeconds<=0) throw new Exception("Invalid input rectangle");
    uint foreground; GetWindowThreadProcessId(GetForegroundWindow(),out foreground);
    if (foreground!=processId) throw new Exception("Gallery does not own foreground");
    var log = new List<string>();
    long frequency=Stopwatch.Frequency, start=Stopwatch.GetTimestamp();
    log.Add("frequency,"+frequency+",start_qpc,"+start);
    log.Add("qpc,tag,kind,x,y,delta");
    var timer=CreateWaitableTimerEx(IntPtr.Zero,null,2,0x00100002);
    if(timer==IntPtr.Zero) throw new Exception("High-resolution timer unavailable");
    int sx=GetSystemMetrics(76),sy=GetSystemMetrics(77),sw=GetSystemMetrics(78),sh=GetSystemMetrics(79);
    POINT previous; GetCursorPos(out previous);
    int moveStep=0, wheelStep=0, sequence=0;
    try {
      while(moveStep<seconds*125 || wheelStep<seconds*60) {
        // Independent deadlines: sampling 60 Hz wheel events inside a 125 Hz
        // pointer loop would introduce an artificial 24 ms wheel gap.
        long moveDeadline=moveStep<seconds*125 ? start+(long)(moveStep*frequency/125.0) : long.MaxValue;
        long wheelDeadline=wheelStep<seconds*60 ? start+(long)(wheelStep*frequency/60.0) : long.MaxValue;
        long deadline=Math.Min(moveDeadline,wheelDeadline), now=Stopwatch.GetTimestamp();
        if(now<deadline) { long due=-(long)((deadline-now)*10000000.0/frequency); SetWaitableTimer(timer,ref due,0,IntPtr.Zero,IntPtr.Zero,false); WaitForSingleObject(timer,1000); }
        bool sendMove=moveDeadline==deadline, sendWheel=wheelDeadline==deadline;
        POINT current; GetCursorPos(out current);
        if(Math.Abs(current.x-previous.x)>3 || Math.Abs(current.y-previous.y)>3 || GetAsyncKeyState(1)<0 || GetAsyncKeyState(2)<0)
          throw new Exception("External pointer/button activity: stopped input");
        GetWindowThreadProcessId(GetForegroundWindow(),out foreground);
        if(foreground!=processId) throw new Exception("Foreground changed: stopped input");
        double elapsed=(Stopwatch.GetTimestamp()-start)/(double)frequency;
        int x=sendMove ? (int)Math.Round((left+right)/2.0+(right-left)*0.46*Math.Sin(elapsed*2*Math.PI/2.3)) : current.x;
        int y=sendMove ? (int)Math.Round((top+bottom)/2.0+(bottom-top)*0.42*Math.Sin(elapsed*2*Math.PI/3.1)) : current.y;
        if(Owner(x,y)!=processId || Owner(current.x,current.y)!=processId) throw new Exception("Pointer target is not owned by Gallery");
        int delta=((int)(elapsed/reverseSeconds)%2==0)?-30:30;
        var inputs=new INPUT[(sendMove?1:0)+(sendWheel?1:0)];
        long tag=0x53490000L+sequence++*2;
        if(sendMove) inputs[0].mouse=new MOUSEINPUT{dx=(int)Math.Round((x-sx)*65535.0/(sw-1)),dy=(int)Math.Round((y-sy)*65535.0/(sh-1)),flags=0x0001|0x8000|0x4000,extra=new IntPtr(tag)};
        if(sendWheel) inputs[sendMove?1:0].mouse=new MOUSEINPUT{data=unchecked((uint)delta),flags=0x0800,extra=new IntPtr(tag+1)};
        now=Stopwatch.GetTimestamp();
        if(SendInput((uint)inputs.Length,inputs,Marshal.SizeOf(typeof(INPUT)))!=inputs.Length) throw new Exception("SendInput failed");
        if(sendMove) { log.Add(now+","+tag+",move,"+x+","+y+",0"); moveStep++; }
        if(sendWheel) { log.Add(now+","+(tag+1)+",wheel,"+x+","+y+","+delta); wheelStep++; }
        previous=new POINT{x=x,y=y};
      }
      log.Add("complete,"+Stopwatch.GetTimestamp());
    } finally { CloseHandle(timer); File.WriteAllLines(path,log); }
  }
}
'@

$prefix=[IO.Path]::GetFullPath($OutputPrefix)
$treePath=$prefix+'-tree.json'
$previousNativeTrace=$env:NATIVEAPI_MESSAGE_TRACE_PATH
$previousTrace=$env:SWIFTUI_PORT_TRACE_PATH
$previousTree=$env:SWIFTUI_PORT_TREE_PATH
$gallery=$null; $probe=$null
try {
  $env:NATIVEAPI_MESSAGE_TRACE_PATH=$prefix+'-native.csv'
  $env:SWIFTUI_PORT_TRACE_PATH=$prefix+'-trace.csv'
  $env:SWIFTUI_PORT_TREE_PATH=$treePath
  $gallery=Start-Process -FilePath $exe -ArgumentList '-design-sizing','-page','button' -WorkingDirectory $exampleRoot -WindowStyle Hidden -PassThru
  for($attempt=0;$attempt -lt 100;$attempt++) {
    Start-Sleep -Milliseconds 100
    $gallery.Refresh()
    if($gallery.HasExited) { throw 'Gallery exited during startup' }
    if($gallery.MainWindowHandle -and (Test-Path -LiteralPath $treePath)) { break }
  }
  $win=@(Get-AppWindows $gallery.Id | Where-Object { $_.ClientW -gt 100 })[0]
  if(!$win) { throw 'Gallery window not found' }
  [WInput]::Move($win.Hwnd,100,40)
  [WInput]::Raise($win.Hwnd)
  [WInput]::Focus($win.Hwnd)
  Start-Sleep -Milliseconds 400
  $win=@(Get-AppWindows $gallery.Id | Where-Object { $_.ClientW -gt 100 })[0]
  function Flatten($node) { $node; foreach($child in $node.children) { Flatten $child } }
  $tree=Get-Content -Raw -LiteralPath $treePath | ConvertFrom-Json
  $scroll=@(Flatten $tree | Where-Object { $_.kind -eq 'ScrollView' -and $_.x -gt 270 -and $_.x -lt 500 })[0]
  if(!$scroll -or !$scroll.children.Count) { throw 'Main Gallery viewport not found in render tree' }
  $left=[int]($win.ClientX+($scroll.x+25)*$win.Scale)
  $right=[int]($win.ClientX+($scroll.x+$scroll.width-25)*$win.Scale)
  $top=[int]($win.ClientY+($scroll.y+80)*$win.Scale)
  $bottom=[int]($win.ClientY+($scroll.y+$scroll.height-80)*$win.Scale)
  $reverse=[Math]::Min(1.2,[Math]::Max(0.2,($scroll.children[0].height-$scroll.height)*0.6/720))
  Assert-Idle
  [WInput]::Glide([int](($left+$right)/2),[int](($top+$bottom)/2),500)
  Assert-Owner $gallery.Id ([int](($left+$right)/2)) ([int](($top+$bottom)/2))
  $probe=Start-Process -FilePath $probeExe -ArgumentList ('"'+$win.Title+'"'),($prefix+'-capture.jsonl'),([string]($Seconds+8)) -WindowStyle Hidden -RedirectStandardOutput ($prefix+'-probe.log') -RedirectStandardError ($prefix+'-probe-error.log') -PassThru
  Start-Sleep -Milliseconds 1000
  [InteractionDriver]::Run($gallery.Id,$left,$top,$right,$bottom,$reverse,$Seconds,($prefix+'-input.csv'))
  Start-Sleep -Milliseconds 1200
} finally {
  $env:NATIVEAPI_MESSAGE_TRACE_PATH=$previousNativeTrace
  $env:SWIFTUI_PORT_TRACE_PATH=$previousTrace
  $env:SWIFTUI_PORT_TREE_PATH=$previousTree
  if($gallery -and !$gallery.HasExited) {
    $gallery.CloseMainWindow() | Out-Null
    if(!$gallery.WaitForExit(10000)) { throw 'Gallery did not exit cleanly; trace is incomplete' }
  }
  if($probe) { $probe.WaitForExit(5000) | Out-Null }
}
Write-Output "Completed normal Example.exe interaction: $prefix"
