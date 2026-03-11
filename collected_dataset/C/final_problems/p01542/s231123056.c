#include <stdio.h>

#define NUM_LIMIT (1<<10)

#define OP_ADD   (-1)
#define OP_SUB   (-2)
#define OP_MUL   (-3)
#define OP_KAKKO (-4)

int stack_num;
int stack[100];
int rp_num;
int rp[100];

int rp_compile(const char* statement) {
	const char* now;
	int expectOperator=0;
	int nownumber;
	stack_num=0;
	rp_num=0;
	for(now=statement;*now;now++) {
		switch(*now) {
			case '+':
				if(!expectOperator)return 0;
				while(stack_num>0 && stack[stack_num-1]!=OP_KAKKO) {
					rp[rp_num++]=stack[--stack_num];
				}
				stack[stack_num++]=OP_ADD;
				expectOperator=0;
				break;
			case '-':
				if(!expectOperator)return 0;
				while(stack_num>0 && stack[stack_num-1]!=OP_KAKKO) {
					rp[rp_num++]=stack[--stack_num];
				}
				stack[stack_num++]=OP_SUB;
				expectOperator=0;
				break;
			case '*':
				if(!expectOperator)return 0;
				while(stack_num>0 && stack[stack_num-1]==OP_MUL) {
					rp[rp_num++]=stack[--stack_num];
				}
				stack[stack_num++]=OP_MUL;
				expectOperator=0;
				break;
			case '(':
				if(expectOperator)return 0;
				stack[stack_num++]=OP_KAKKO;
				expectOperator=0;
				break;
			case ')':
				if(!expectOperator)return 0;
				if(stack_num<=0 || stack[stack_num-1]==OP_KAKKO)return 0;
				while(stack_num>0 && stack[stack_num-1]!=OP_KAKKO) {
					rp[rp_num++]=stack[--stack_num];
				}
				if(stack_num>0)stack_num--; else return 0;
				expectOperator=1;
				break;
			case '0':
			case '1':
				if(expectOperator)return 0;
				nownumber=0;
				for(;*now=='0' || *now=='1';now++) {
					nownumber=nownumber*2+(*now)-'0';
					if(nownumber>=NUM_LIMIT)return 0;
				}
				now--;
				expectOperator=1;
				rp[rp_num++]=nownumber;
				break;
		}
	}
	while(stack_num>0) {
		stack_num--;
		if(stack[stack_num]==OP_KAKKO)return 0;
		rp[rp_num++]=stack[stack_num];
	}
	return 1;
}

int rp_calc(void) {
	int i;
	stack_num=0;
	for(i=0;i<rp_num;i++) {
		switch(rp[i]) {
			case OP_ADD:
				if(stack_num<2)return -1;
				stack[stack_num-2]+=stack[stack_num-1];
				if(stack[stack_num-2]>=NUM_LIMIT)return -1;
				stack_num--;
				break;
			case OP_SUB:
				if(stack_num<2)return -1;
				stack[stack_num-2]-=stack[stack_num-1];
				if(stack[stack_num-2]<0)return -1;
				stack_num--;
				break;
			case OP_MUL:
				if(stack_num<2)return -1;
				stack[stack_num-2]*=stack[stack_num-1];
				if(stack[stack_num-2]>=NUM_LIMIT)return -1;
				stack_num--;
				break;
			default:
				stack[stack_num++]=rp[i];
				break;
		}
	}
	if(stack_num!=1)return -1;
	return stack[0];
}

int main(void) {
	char input[200];
	const char* mozi="01+-*()";
	int pos[5]={150,150,150,150,150};
	int i,j;
	int posnum;
	int search_max;
	int result;
	scanf("%s",input);
	posnum=0;
	search_max=1;
	for(i=0;input[i];i++) {
		if(input[i]=='.') {
			pos[posnum++]=i;
			search_max*=7;
		}
	}
	result=-1;
	for(i=0;i<search_max;i++) {
		int temp=i;
		for(j=0;j<posnum;j++) {
			input[pos[j]]=mozi[temp%7];
			temp/=7;
		}
		if(!rp_compile(input))continue;
		temp=rp_calc();
		if(temp>result)result=temp;
	}
	printf("%d\n",result);
	return 0;
}