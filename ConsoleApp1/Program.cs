using System;
using System.Runtime.InteropServices;

namespace ConsoleApp1
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SampleStruct
    {
        public Int32 iterations;
        public Int64 duration;
    }

    class Program
    {
        static void Main(string[] args)
        {
            //Console.WriteLine("Hello World!");
            var addedNumbers = add_numbers(10, 5);
            Console.WriteLine(addedNumbers);
            //var h = hello_world();
            //Console.WriteLine(Marshal.PtrToStringAnsi(h));
            //free_string();
            var d = DateTime.Now;
            //var l = test_loop();
            var t = time();
            var f = DateTime.Now;
            
            Console.WriteLine("Inside Rust: {0} - {1}ms", t.iterations, t.duration);
            Console.WriteLine("Outside Rust: {0}ms", (f - d).TotalMilliseconds);

            d = DateTime.Now;
            var sum = 0;
            for (int i = 0; i < 1000000000; i++)
            {
                sum = i;
            }
            f = DateTime.Now;
            Console.WriteLine("C# loop {0} - {1}ms", sum, (f - d).TotalMilliseconds);
            Console.ReadLine();
        }

        [DllImport("libs/rustcsharp.dll")]
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);

        [DllImport("libs/rustcsharp.dll")]
        private static extern IntPtr hello_world();

        [DllImport("libs/rustcsharp.dll")]
        private static extern Int32 test_loop();

        [DllImport("libs/rustcsharp.dll")]
        private static extern void free_string();

        [DllImport("libs/rustcsharp.dll")]
        private static extern SampleStruct time();
    }
}
