#include <stdio.h>

char input[10000];
int decode_num;
int input_decode[1000];

int main(void) {
	int i;
	int prev;
	int max,last;
	int now;
	scanf("%s",input);
	decode_num=0;
	for(i=0;input[i];) {
		if(input[i]=='e') {
			input_decode[decode_num++]=0;
			i+=3;
		} else if(input[i]=='c') {
			input_decode[decode_num++]=1;
			i+=7;
		} else return 1;
	}
	max=last=0;
	now=0;
	prev=100;
	for(i=0;i<decode_num;i++) {
		if(input_decode[i]+prev!=1) {
			if(now>max) {
				max=now;
				last=prev;
				now=0;
			}
		}
		prev=input_decode[i];
		now++;
	}
	if(now>max)last=prev;
	puts(last==0?"egg":"chicken");
	return 0;
}