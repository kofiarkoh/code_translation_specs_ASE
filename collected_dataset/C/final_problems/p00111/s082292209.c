#include <stdio.h>
#include <string.h>
#define NUM_CODE 32
#define BUF_SIZE 20

const char *code_table['Z'+1] = {
	['A']="00000", ['B']="00001", ['C']="00010", ['D']="00011",
	['E']="00100", ['F']="00101", ['G']="00110", ['H']="00111",
	['I']="01000", ['J']="01001", ['K']="01010", ['L']="01011",
	['M']="01100", ['N']="01101", ['O']="01110", ['P']="01111",
	['Q']="10000", ['R']="10001", ['S']="10010", ['T']="10011",
	['U']="10100", ['V']="10101", ['W']="10110", ['X']="10111",
	['Y']="11000", ['Z']="11001", [' ']="11010", ['.']="11011",
	[',']="11100", ['-']="11101", ['\'']="11110", ['?']="11111"
};

const char *decrypt_table[NUM_CODE] = {
	"100101", "10011010", "0101", "0001", "110", "01001", "10011011",
	"010000", "0111", "10011000", "0110", "00100", "10011001",
	"10011110", "00101", "111", "10011111", "1000", "00110", "00111",
	"10011100", "10011101", "000010", "10010010", "10010011", "10010000",
	"101", "000000", "000011", "10010001", "010001", "000001"
};

int main(void) {
	int c, k;
	size_t i, len;
	char buf[BUF_SIZE] = "";

	while (1) {
		c = getchar();
		if (c == '\r') {
			c = getchar();
		}
		if (c == '\n') {
			putchar('\n');
			memset(buf, '\0', BUF_SIZE);
			continue;
		}
		if (c == EOF)
			break;

		strcat(buf, code_table[c]);

		while (1) {
			for (i = 0; i < NUM_CODE; i++) {
				for (k = 0; decrypt_table[i][k]; k++) {
					if (decrypt_table[i][k] != buf[k]) {
						k = -1;
						break;
					}
				}
				if (k >= 0) {
					break;
				}
			}
			if (i >= NUM_CODE)
				break;

			putchar("ABCDEFGHIJKLMNOPQRSTUVWXYZ ',-.?"[i]);

			len = strlen(buf+k);
			memmove(buf, buf+k, len);
			buf[len] = '\0';
		}
	}
	return 0;
}