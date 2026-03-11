#include <stdio.h>
#include <string.h>

int jump[100];
char source[100][20];

int visited[101];

void tansaku(int pos,int cost,int N) {
	if(visited[pos]>=0 && visited[pos]<=cost)return;
	visited[pos]=cost;
	if(jump[pos]==0) {
		if(pos<N)tansaku(pos+1,cost,N);
	} else {
		tansaku(jump[pos]-1,cost,N);
		if(pos<N)tansaku(pos+1,cost+1,N);
	}
}

int main(void) {
	int N,i,j,k;
	char buffer[100];
	int groupid;
	scanf("%d",&N);
	for(i=0;i<N;i++) {
		scanf("%s",buffer);
		if(strcmp(buffer,"goto")==0) {
			jump[i]=1;
			scanf("%s",source[i]);
			source[i][strlen(source[i])-1]=0;
		} else {
			jump[i]=0;
			strcpy(source[i],buffer);
			source[i][strlen(source[i])-1]=0;
		}
	}
	for(i=0;i<N;i++) {
		if(jump[i]) {
			for(j=0;j<N;j++) {
				if(i==j)continue;
				if(jump[j]==0 && strcmp(source[i],source[j])==0) {
					jump[i]=j+1;
					break;
				}
			}
		}
	}
	for(i=0;i<=N;i++)visited[i]=-1;
	tansaku(0,0,N);
	printf("%d\n",visited[N]);
	return 0;
}