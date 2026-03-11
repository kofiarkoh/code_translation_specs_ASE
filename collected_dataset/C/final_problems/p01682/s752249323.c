#include <stdio.h>
#include <ctype.h>

#define MOD_BY 1000000007
#define SIZE_MAX 2000000

#define ESU (-1)
#define SIFUTO (-2)
#define KAKKO (-3)

int expr[SIZE_MAX];
int expr_num;
int stack[SIZE_MAX];
int stack_num;

void compile(const char* expr_str) {
	int i;
	int digit_flag=0;
	int current_number=0;
	expr_num=stack_num=0;
	for(i=0;expr_str[i]!='\0';i++) {
		if(isdigit(expr_str[i])) {
			current_number=current_number*10+expr_str[i]-'0';
			digit_flag=1;
		} else {
			if(digit_flag) {
				expr[expr_num++]=current_number;
				current_number=0;
				digit_flag=0;
			}
			switch(expr_str[i]) {
				case 'S':
					stack[stack_num++]=ESU;
					break;
				case '<':
					stack[stack_num++]=KAKKO;
					break;
				case '>':
					while(stack_num>0 && stack[stack_num-1]!=KAKKO) {
						expr[expr_num++]=stack[--stack_num];
					}
					stack[stack_num++]=SIFUTO;
					i++;
					break;
				case ')':
					while(stack_num>0 && stack[--stack_num]!=KAKKO) {
						expr[expr_num++]=stack[stack_num];
					}
					break;
			}
		}
	}
	if(digit_flag) {
		expr[expr_num++]=current_number;
		digit_flag=0;
	}
	while(stack_num>0) {
		expr[expr_num++]=stack[--stack_num];
	}
}

int run(void) {
	int i;
	stack_num=0;
	for(i=0;i<expr_num;i++) {
		if(expr[i]==ESU) {
			int ret=0,now=stack[stack_num-1],left=stack[stack_num-1];
			while(left>0) {
				if(left&1) {
					ret+=now;
					if(ret>=MOD_BY)ret-=MOD_BY;
				}
				now+=now;
				if(now>=MOD_BY)now-=MOD_BY;
				left>>=1;
			}
			stack[stack_num-1]=ret;
		} else if(expr[i]==SIFUTO) {
			if(stack[stack_num-1]<32) {
				stack[stack_num-2]>>=stack[stack_num-1];
			} else {
				stack[stack_num-2]=0;
			}
			stack_num--;
		} else if(expr[i]>=0) {
			stack[stack_num++]=expr[i];
		}
	}
	return stack[0];
}

int main(void) {
	static char input[SIZE_MAX];
	while(fgets(input,sizeof(input),stdin)!=NULL && input[0]!='#') {
		int i;
		int expect_number=1;
		for(i=0;input[i];i++);
		for(i--;i>=0;i--) {
			if(input[i]=='>') {
				if(expect_number) {
					input[i]=')';
				} else {
					i--;
					expect_number=1;
				}
			} else if(isdigit(input[i])) {
				expect_number=0;
			} else if(input[i]=='S') {
				expect_number=0;
			}
		}
		compile(input);
		printf("%d\n",run());
	}
	return 0;
}