module full_add(a, b, cin, sum, cout);
  input a, b, cin;
  output sum, cout;
  wire x, y, z;

  half_add h1(.a(a), .b(b), .s(x), .c(y));
  half_add h2(.a(x), .b(cin), .s(sum), .c(z));
  or o1(cout, y, z);
endmodule : full_add

module half_add(a,b,s,c);
  input a, b;
  output s, c;

  xor x1(s, a, b);
  and a1(c, a, b);
endmodule : half_add
