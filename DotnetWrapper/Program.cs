using System;
using DotnetWrapper;

Console.WriteLine("Testing Rust Text Analyzer Core...\n");

string[] testCases = new string[]
{
    "https://www.google.com",
    "SELECT * FROM users WHERE age > 21",
    "{\"name\": \"John\", \"age\": 30}",
    "user@example.com",
    "192.168.1.1",
    "console.log('Hello World');",
    "2026-08-26T16:25:23+05:00",
    "# This is a comment in Python",
};

foreach (var text in testCases)
{
    string result = TextAnalyzer.Analyze(text);
    Console.WriteLine($"Input: {text}");
    Console.WriteLine($"Result: {result}\n");
}
