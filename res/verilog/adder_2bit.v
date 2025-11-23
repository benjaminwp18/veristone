module add_2bit(
  input a1,
  input a2,
  input b1,
  input b2,
  input cin,
  output s1,
  output s2,
  output cout
);
  wire x;

  full_add add1(.a(a1), .b(b1), .cin(cin), .sum(s1), .cout(x));
  full_add add2(.a(a2), .b(b2), .cin(x), .sum(s2), .cout(cout));
endmodule : add_2bit

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
