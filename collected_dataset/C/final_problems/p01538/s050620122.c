#include <stdio.h>
#include <string.h>

char visited[1000001];

int str2num(const char* str,int len) {
	int result,i;
	result=0;
	for(i=0;i<len;i++)result=result*10+str[i]-'0';
	return result;
}

int getnextnum(int num) {
	char thenumber[100];
	int length;
	int max;
	int i;
	sprintf(thenumber,"%d",num);
	length=strlen(thenumber);
	max=0;
	for(i=1;i<length;i++) {
		int temp;
		temp=str2num(thenumber,i)*str2num(&thenumber[i],length-i);
		if(max<temp)max=temp;
	}
	return max;
}

int main(void) {
	int Q,i;
	int N;
	int count;
	scanf("%d",&Q);
	for(i=0;i<Q;i++) {
		scanf("%d",&N);
		memset(visited,0,sizeof(visited));
		for(count=0;N>=10;count++) {
			if(visited[N]){count=-1;break;}
			visited[N]=1;
			N=getnextnum(N);
		}
		printf("%d\n",count);
	}
	return 0;
}