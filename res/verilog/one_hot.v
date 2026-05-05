module one_hot(input [1:0] binary, output [3:0] one_hot);
    assign one_hot[0] = ~binary[0] & ~binary[1];
    assign one_hot[1] = binary[0] & ~binary[1];
    assign one_hot[2] = ~binary[0] & binary[1];
    assign one_hot[3] = binary[0] & binary[1];
endmodule