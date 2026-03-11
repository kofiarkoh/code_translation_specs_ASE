#include <stdio.h>

#define OP_NULL		100
#define OP_ADD		200
#define OP_SUB		300
#define OP_MUL		400
#define OP_EQUAL	500

int calc(int R1,int R2,int R3) {
	switch(R3) {
		case OP_NULL:
			return R2;
		case OP_ADD:
			return R1+R2;
		case OP_SUB:
			return R1-R2;
		case OP_MUL:
			return R1*R2;
		default:
			return -1;
	}
}

int main(void) {
	char input[100];
	int R1,R2,R3;
	int i,error;
	while(scanf("%s",input)==1) {
		R1=R2=0;
		R3=OP_NULL;
		error=0;
		for(i=0;input[i];i++) {
			if(error && input[i]!='=')continue;
			switch(input[i]) {
				case '+':
					R1=calc(R1,R2,R3);
					if(R1<0 || R1>9999)error=1;
					R2=0;
					R3=OP_ADD;
					break;
				case '-':
					R1=calc(R1,R2,R3);
					if(R1<0 || R1>9999)error=1;
					R2=0;
					R3=OP_SUB;
					break;
				case '*':
					R1=calc(R1,R2,R3);
					if(R1<0 || R1>9999)error=1;
					R2=0;
					R3=OP_MUL;
					break;
				case '=':
					R1=calc(R1,R2,R3);
					if(R1<0 || R1>9999)error=1;
					if(error) {
						puts("E");
						error=0;
					} else printf("%d\n",R1);
					R2=0;
					R3=OP_NULL;
					break;
				default:
					R2=R2*10+(input[i]-'0');
					if(R2<0 || R2>9999)error=1;
			}
		}
		if(error)puts("E");
	}
	return 0;
}