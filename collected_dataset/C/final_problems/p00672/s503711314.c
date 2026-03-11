#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define OPERATOR_NUMBER	0
#define OPERATOR_PLUS	1
#define OPERATOR_MINUS	2
#define OPERATOR_MUL	3
#define OPERATOR_DIV	4
#define OPERATOR_KAKKO	5

typedef struct {
	char name[24];
	int zigen[5];
} tani_t;

typedef struct {
	char name[24];
	int zigen[5];
} hensuu_t;

int zigen_num;
int tani_num;
tani_t tani[10];
int hensuu_num;
hensuu_t hensuu[15];

int main_stack_num;
int main_stack[100][6];
int operator_stack_num;
int operator_stack[100];
int calc_stack_num;
int calc_stack[100][5];

int main(void) {
	int i,j,k;
	char suusiki[200];
	char now_tani[24];
	char now_hensuu[24];
	int iserror;
	while(1) {
		scanf("%d%d%d",&zigen_num,&tani_num,&hensuu_num);
		if((zigen_num|tani_num|hensuu_num)==0)break;
		for(i=0;i<tani_num;i++) {
			scanf("%s",tani[i].name);
			for(j=0;j<zigen_num;j++)scanf("%d",&tani[i].zigen[j]);
		}
		scanf("%s",suusiki);
		for(i=0;i<hensuu_num;i++) {
			scanf("%s%s",hensuu[i].name,now_tani);
			for(j=0;j<tani_num;j++) {
				if(strcmp(tani[j].name,now_tani)==0) {
					for(k=0;k<zigen_num;k++) {
						hensuu[i].zigen[k]=tani[j].zigen[k];
					}
					break;
				}
			}
		}
		main_stack_num=operator_stack_num=calc_stack_num=0;
		for(i=0;suusiki[i];i++) {
			if(suusiki[i]=='+') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]!=OPERATOR_KAKKO) {
					main_stack[main_stack_num++][5]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_PLUS;
			} else if(suusiki[i]=='-') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]!=OPERATOR_KAKKO) {
					main_stack[main_stack_num++][5]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_MINUS;
			} else if(suusiki[i]=='*') {
				while(operator_stack_num>0 &&
						(operator_stack[operator_stack_num-1]==OPERATOR_MUL ||
						operator_stack[operator_stack_num-1]==OPERATOR_DIV)) {
					main_stack[main_stack_num++][5]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_MUL;
			} else if(suusiki[i]=='/') {
				while(operator_stack_num>0 &&
						(operator_stack[operator_stack_num-1]==OPERATOR_MUL ||
						operator_stack[operator_stack_num-1]==OPERATOR_DIV)) {
					main_stack[main_stack_num++][5]=operator_stack[--operator_stack_num];
				}
				operator_stack[operator_stack_num++]=OPERATOR_DIV;
			} else if(suusiki[i]=='(') {
				operator_stack[operator_stack_num++]=OPERATOR_KAKKO;
			} else if(suusiki[i]==')') {
				while(operator_stack_num>0 &&
						operator_stack[operator_stack_num-1]!=OPERATOR_KAKKO) {
					main_stack[main_stack_num++][5]=operator_stack[--operator_stack_num];
				}
				if(operator_stack_num<=0)return 1;
				operator_stack_num--;
			} else if(isalpha(suusiki[i])) {
				for(j=i;isalpha(suusiki[j]);j++);
				if(j-i>20)return 1;
				memcpy(now_hensuu,&suusiki[i],j-i);
				now_hensuu[j-i]=0;
				i=j-1;
				for(j=0;j<hensuu_num;j++) {
					if(strcmp(now_hensuu,hensuu[j].name)==0) {
						for(k=0;k<zigen_num;k++) {
							main_stack[main_stack_num][k]=hensuu[j].zigen[k];
						}
						main_stack[main_stack_num++][5]=OPERATOR_NUMBER;
						break;
					}
				}
				if(j>=hensuu_num)return 1;
			} else return 1;
		}
		while(operator_stack_num>0) {
			if(operator_stack[operator_stack_num-1]==OPERATOR_KAKKO)return 1;
			main_stack[main_stack_num++][5]=operator_stack[--operator_stack_num];
		}
		iserror=0;
		for(i=0;i<main_stack_num;i++) {
			switch(main_stack[i][5]) {
				case OPERATOR_NUMBER:
					for(j=0;j<zigen_num;j++) {
						calc_stack[calc_stack_num][j]=main_stack[i][j];
					}
					calc_stack_num++;
					break;
				case OPERATOR_PLUS:
				case OPERATOR_MINUS:
					if(calc_stack_num<2)return 1;
					for(j=0;j<zigen_num;j++) {
						if(calc_stack[calc_stack_num-2][j]!=calc_stack[calc_stack_num-1][j]) {
							iserror=1;
							i=main_stack_num;
							break;
						}
					}
					calc_stack_num--;
					break;
				case OPERATOR_MUL:
					if(calc_stack_num<2)return 1;
					for(j=0;j<zigen_num;j++) {
						calc_stack[calc_stack_num-2][j]+=calc_stack[calc_stack_num-1][j];
					}
					calc_stack_num--;
					break;
				case OPERATOR_DIV:
					if(calc_stack_num<2)return 1;
					for(j=0;j<zigen_num;j++) {
						calc_stack[calc_stack_num-2][j]-=calc_stack[calc_stack_num-1][j];
					}
					calc_stack_num--;
					break;
				default:
					return 1;
			}
		}
		if(calc_stack_num!=1 && !iserror)return 1;
		if(iserror) {
			puts("error");
		} else {
			iserror=1;
			for(i=tani_num-1;i>=0;i--) {
				for(j=0;j<zigen_num;j++) {
					if(tani[i].zigen[j]!=calc_stack[0][j])break;
				}
				if(j>=zigen_num) {
					puts(tani[i].name);
					iserror=0;
					break;
				}
			}
			if(iserror)puts("undefined");
		}
	}
	return 0;
}