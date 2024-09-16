using System;
using System.Runtime.InteropServices;

Console.WriteLine($"Hello Swetugg '24 Göteborg from {RuntimeInformation.OSArchitecture}!");

var text = System.IO.File.ReadAllText("/etc/hosts");
Console.WriteLine(text);