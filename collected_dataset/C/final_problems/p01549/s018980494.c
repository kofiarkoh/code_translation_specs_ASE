#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	char arierukamo[256];
} num_t;

num_t create_num(int l,int u) {
	num_t ret;
	int i;
	for(i=0;i<256;i++)ret.arierukamo[i]=(l<=i && i<=u)?1:0;
	return ret;
}

typedef int (*p_op)(int,int);

int add(int a,int b){return a+b;}
int sub(int a,int b){return a-b+256;}
int mul(int a,int b){return a*b;}
int waru(int a,int b){return a/b;}

num_t calc_num(const num_t *a,const num_t *b,p_op op) {
	num_t ret;
	int i,j;
	for(i=0;i<256;i++)ret.arierukamo[i]=0;
	for(i=0;i<256;i++) {
		for(j=0;j<256;j++) {
			if(a->arierukamo[i] && b->arierukamo[j]) {
				ret.arierukamo[op(i,j)%256]=1;
			}
		}
	}
	return ret;
}

int main(void) {
	int m,n,i;
	char names[100][32];
	num_t vars[100];
	num_t stack[100];
	int stack_pos=0;
	int error_flag=0;
	if(scanf("%d",&m)!=1)return 1;
	for(i=0;i<m;i++) {
		int l,u;
		if(scanf("%s%d%d",names[i],&l,&u)!=3)return 1;
		vars[i]=create_num(l,u);
	}
	if(scanf("%d",&n)!=1)return 1;
	for(i=0;i<n;i++) {
		char data[1024];
		char* end;
		int num;
		if(scanf("%s",data)!=1)return 1;
		if(error_flag)continue;
		num=(int)strtol(data,&end,10);
		if(*end=='\0') {
			stack[stack_pos++]=create_num(num,num);
		} else if(strcmp(data,"+")==0) {
			if(stack_pos<2)error_flag=1;
			else {
				stack_pos--;
				stack[stack_pos-1]=calc_num(&stack[stack_pos-1],&stack[stack_pos],add);
			}
		} else if(strcmp(data,"-")==0) {
			if(stack_pos<2)error_flag=1;
			else {
				stack_pos--;
				stack[stack_pos-1]=calc_num(&stack[stack_pos-1],&stack[stack_pos],sub);
			}
		} else if(strcmp(data,"*")==0) {
			if(stack_pos<2)error_flag=1;
			else {
				stack_pos--;
				stack[stack_pos-1]=calc_num(&stack[stack_pos-1],&stack[stack_pos],mul);
			}
		} else if(strcmp(data,"/")==0) {
			if(stack_pos<2 || stack[stack_pos-1].arierukamo[0]!=0)error_flag=1;
			else {
				stack_pos--;
				stack[stack_pos-1]=calc_num(&stack[stack_pos-1],&stack[stack_pos],waru);
			}
		} else {
			int j;
			int index=-1;
			for(j=0;j<m;j++) {
				if(strcmp(data,names[j])==0) {
					index=j;
					break;
				}
			}
			if(index<0)error_flag=1; else stack[stack_pos++]=vars[j];
		}
	}
	if(stack_pos!=1)error_flag=1;
	puts(error_flag?"error":"correct");
	return 0;
}