#include <stdio.h>
#include <string.h>

int main(void) {
	int num;
	char input1[9];
	char input2[9];
	char output[9];
	output[0] = '\0';
	int change1;
	int change2;
	int count1;
	int count2;

	scanf("%d", &num);
	for (count1 = 0; count1 < num; count1++) {
		change1 = 0;
		change2 = 0;
		scanf("%s", input1);
		scanf("%s", input2);
		for (count2 = 0; count2 < strlen(input1); count2++) {
			if (input1[count2] == 'm')
				change1 = 1000;
			else if (input1[count2] == 'c')
				change1 += 100;
			else if (input1[count2] == 'x')
				change1 += 10;
			else if (input1[count2] == 'i') {
				change1 += 1;
				break;
			}
			else {
				if (input1[count2 + 1] == 'm')
					change1 += (input1[count2]-'0') * 1000;
				else if (input1[count2 + 1] == 'c')
					change1 += (input1[count2]-'0') * 100;
				else if (input1[count2 + 1] == 'x')
					change1 += (input1[count2]-'0') * 10;
				else {
					change1 += input1[count2]-'0';
					break;
				}
				count2++;
			}
		}
		for (count2 = 0; count2 < strlen(input2); count2++) {
			if (input2[count2] == 'm')
				change2 = 1000;
			else if (input2[count2] == 'c')
				change2 += 100;
			else if (input2[count2] == 'x')
				change2 += 10;
			else if (input2[count2] == 'i') {
				change2 += 1;
				break;
			}
			else {
				if (input2[count2 + 1] == 'm')
					change2 += (input2[count2]-'0') * 1000;
				else if (input2[count2 + 1] == 'c')
					change2 += (input2[count2]-'0') * 100;
				else if (input2[count2 + 1] == 'x')
					change2 += (input2[count2]-'0') * 10;
				else {
					change2 += input2[count2]-'0';
					break;
				}
				count2++;
			}
		}
		count2 = 0;
		change1 += change2;
		if (change1 / 1000 > 1) {
			output[0]= '0' + change1 / 1000;
			output[1] = 'm';
			count2 = 2;
			change1 = change1 % 1000;
		}
		else if (change1 / 1000 == 1) {
			output[0]= 'm';
			count2 = 1;
			change1 = change1 - 1000;
		}
		if (change1 / 100 > 1) {
			output[count2]= '0' + change1 / 100;
			output[count2 + 1] = 'c';
			count2 += 2;
			change1 = change1 % 100;
		}
		else if (change1 / 100 == 1) {
			output[count2]= 'c';
			count2++;
			change1 = change1 - 100;
		}
		if (change1 / 10 > 1) {
			output[count2]= '0' + change1 / 10;
			output[count2+1]= 'x';
			count2 += 2;
			change1 = change1 % 10;
		}
		else if (change1 / 10 == 1) {
			output[count2]= 'x';
			count2++;
			change1 = change1 - 10;
		}
		if (change1 > 1) {
			output[count2]= '0' + change1;
			output[count2+1]= 'i';
			count2 += 2;
		}
		else if (change1 == 1) {
			output[count2]= 'i';
			count2++;
		}
		output[count2]= '\0';
		printf("%s\n", output);
	}
}
