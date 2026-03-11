#include <stdio.h>
#include <ctype.h>

#define OPERATOR_NUMBER	0
#define OPERATOR_ADD	1
#define OPERATOR_SUB	2
#define OPERATOR_MUL	3
#define OPERATOR_KAKKO	4

int main_stack_num;
int main_stack[200][3];
int operator_stack_num;
int operator_stack[200];
int calc_stack_num;
int calc_stack[200][2];

int overflow_check(int calc[2]) {
	return (calc[0]<-10000 || calc[0]>10000 ||
		calc[1]<-10000 || calc[1]>10000);
}

int main(void) {
	int i;
	int overflow;
	char query[200];
	while(scanf("%s",query)==1) {
		overflow=0;
		main_stack_num=operator_stack_num=calc_stack_num=0;
		for(i=0;query[i];i++) {
			if(query[i]=='+') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]!=OPERATOR_KAKKO) {
					main_stack[main_stack_num++][0]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_ADD;
			} else if(query[i]=='-') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]!=OPERATOR_KAKKO) {
					main_stack[main_stack_num++][0]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_SUB;
			} else if(query[i]=='*') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]==OPERATOR_MUL) {
					main_stack[main_stack_num++][0]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_MUL;
			} else if(query[i]=='(') {
				operator_stack[operator_stack_num++]=OPERATOR_KAKKO;
			} else if(query[i]==')') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]!=OPERATOR_KAKKO) {
					main_stack[main_stack_num++][0]=operator_stack[--operator_stack_num];
				}
				operator_stack_num--;
			} else if(query[i]=='i') {
				main_stack[main_stack_num][0]=OPERATOR_NUMBER;
				main_stack[main_stack_num][1]=0;
				main_stack[main_stack_num][2]=1;
				main_stack_num++;
			} else if(isdigit(query[i])) {
				main_stack[main_stack_num][0]=OPERATOR_NUMBER;
				main_stack[main_stack_num][1]=0;
				main_stack[main_stack_num][2]=0;
				for(;isdigit(query[i]);i++) {
					main_stack[main_stack_num][1]*=10;
					main_stack[main_stack_num][1]+=query[i]-'0';
					if(main_stack[main_stack_num][1]>10000) {
						overflow=1;
						break;
					}
				}
				i--;
				main_stack_num++;
			} else return 1;
		}
		while(operator_stack_num>0) {
			if(operator_stack[operator_stack_num]=='(')return 1;
			main_stack[main_stack_num++][0]=operator_stack[--operator_stack_num];
		}
		for(i=0;i<main_stack_num && !overflow;i++) {
			switch(main_stack[i][0]) {
				case OPERATOR_NUMBER:
					calc_stack[calc_stack_num][0]=main_stack[i][1];
					calc_stack[calc_stack_num][1]=main_stack[i][2];
					if(overflow_check(calc_stack[calc_stack_num])) {
						overflow=1;
					}
					calc_stack_num++;
					break;
				case OPERATOR_ADD:
					if(calc_stack_num<2)return 1;
					calc_stack[calc_stack_num-2][0]+=calc_stack[calc_stack_num-1][0];
					calc_stack[calc_stack_num-2][1]+=calc_stack[calc_stack_num-1][1];
					if(overflow_check(calc_stack[calc_stack_num-2])) {
						overflow=1;
					}
					calc_stack_num--;
					break;
				case OPERATOR_SUB:
					if(calc_stack_num<2)return 1;
					calc_stack[calc_stack_num-2][0]-=calc_stack[calc_stack_num-1][0];
					calc_stack[calc_stack_num-2][1]-=calc_stack[calc_stack_num-1][1];
					if(overflow_check(calc_stack[calc_stack_num-2])) {
						overflow=1;
					}
					calc_stack_num--;
					break;
				case OPERATOR_MUL:
					if(calc_stack_num<2)return 1;
					calc_stack[calc_stack_num][0]=
						calc_stack[calc_stack_num-2][0]*calc_stack[calc_stack_num-1][0]-
						calc_stack[calc_stack_num-2][1]*calc_stack[calc_stack_num-1][1];
					calc_stack[calc_stack_num][1]=
						calc_stack[calc_stack_num-2][0]*calc_stack[calc_stack_num-1][1]+
						calc_stack[calc_stack_num-2][1]*calc_stack[calc_stack_num-1][0];
					calc_stack[calc_stack_num-2][0]=calc_stack[calc_stack_num][0];
					calc_stack[calc_stack_num-2][1]=calc_stack[calc_stack_num][1];
					if(overflow_check(calc_stack[calc_stack_num-2])) {
						overflow=1;
					}
					calc_stack_num--;
					break;
				default:
					return 1;
			}
		}
		if(overflow) {
			puts("overflow");
		} else {
			if(calc_stack_num!=1)return 1;
			if(calc_stack[0][0]==0) {
				if(calc_stack[0][1]==0)puts("0");
				else printf("%di\n",calc_stack[0][1]);
			} else {
				if(calc_stack[0][1]) {
					printf("%d%s%di\n",calc_stack[0][0],
						(calc_stack[0][1]<0?"":"+"),calc_stack[0][1]);
				} else {
					printf("%d\n",calc_stack[0][0]);
				}
			}
		}
	}
	return 0;
}