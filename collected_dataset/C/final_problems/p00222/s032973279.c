#include <stdio.h>

#define N_MAX 10000000

char is_sosuu[N_MAX+1];
int query_answer[N_MAX+1];

void make_sosuu_list(void) {
	int i,j;
	for(i=0;i<=N_MAX;i++)is_sosuu[i]=1;
	is_sosuu[0]=is_sosuu[1]=0;
	for(i=2;i<=N_MAX;i++) {
		if(is_sosuu[i]) {
			for(j=i+i;j<=N_MAX;j+=i)is_sosuu[j]=0;
		}
	}
}

void make_query_answer(void) {
	int i,now_answer;
	now_answer=0;
	for(i=13;i<=N_MAX;i++) {
		if(is_sosuu[i] && is_sosuu[i-2] &&
			is_sosuu[i-6] && is_sosuu[i-8])now_answer=i;
		query_answer[i]=now_answer;
	}
}

int main(void) {
	int input;
	make_sosuu_list();
	make_query_answer();
	while(1) {
		scanf("%d",&input);
		if(input==0)break;
		printf("%d\n",query_answer[input]);
	}
	return 0;
}