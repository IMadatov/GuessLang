using System;
using System.Runtime.InteropServices;

namespace GuessLang.Core
{
    public class TextAnalyzer
    {
        private const string LibName = "text_analyzer_core";

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        private static extern IntPtr analyze_text_c(string input);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void free_c_string(IntPtr ptr);

        public static string Analyze(string text)
        {
            IntPtr ptr = analyze_text_c(text);
            if (ptr == IntPtr.Zero)
            {
                return "null";
            }

            try
            {
                string result = Marshal.PtrToStringAnsi(ptr);
                return result;
            }
            finally
            {
                free_c_string(ptr);
            }
        }
    }
}
