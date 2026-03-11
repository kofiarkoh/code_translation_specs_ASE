#include <stdio.h>
#include <string.h>

typedef struct {
	int n,m;
	char c[100];
	int a[5000];
	int b[5000];
	int route[100][5000];
	int routenum[100];
} field_t;

field_t aki;
field_t maki;

void getinput(field_t* field) {
	int i;
	char buf[4];
	memset(field,0,sizeof(field_t));
	scanf("%d%d",&field->n,&field->m);
	for(i=0;i<field->n;i++) {
		scanf("%s",buf);
		field->c[i]=buf[0];
	}
	for(i=0;i<field->m;i++) {
		scanf("%d%d",&field->a[i],&field->b[i]);
		field->a[i]--;field->b[i]--;
		field->route[field->a[i]][field->routenum[field->a[i]]]=field->b[i];
		field->routenum[field->a[i]]++;
	}
}

int visited[100][100];
char nowvisited[100][100];

int tansaku(int a,int m,int score) {
	int maxscore=0;
	int nowscore;
	int addscore;
	int i,j;
	if(nowvisited[a][m]) {
		if(visited[a][m]-1<score)return 1000000;
		return 0;
	}
	if(visited[a][m]>0)return visited[a][m]-1;
	visited[a][m]=score+1;
	nowvisited[a][m]=1;
	if(aki.c[a]==maki.c[m]) {
		addscore=1;
	} else {
		addscore=0;
		for(i=0;i<aki.routenum[a];i++) {
			nowscore=tansaku(aki.route[a][i],m,score);
			if(nowscore>maxscore)maxscore=nowscore;
		}
		for(i=0;i<maki.routenum[m];i++) {
			nowscore=tansaku(a,maki.route[m][i],score);
			if(nowscore>maxscore)maxscore=nowscore;
		}
	}
	for(i=0;i<aki.routenum[a];i++) {
		for(j=0;j<maki.routenum[m];j++) {
			nowscore=tansaku(aki.route[a][i],maki.route[m][j],score+addscore);
			if(nowscore>maxscore)maxscore=nowscore;
		}
	}
	nowvisited[a][m]=0;
	visited[a][m]=maxscore+addscore+1;
	return maxscore+addscore;
}

int main(void) {
	int i,j;
	int maxscore=0;
	int nowscore;
	getinput(&aki);
	getinput(&maki);
	for(i=0;i<aki.n;i++) {
		for(j=0;j<maki.n;j++) {
			if(!visited[i][j]) {
				memset(nowvisited,0,sizeof(nowvisited));
				nowscore=tansaku(i,j,0);
				if(nowscore>maxscore)maxscore=nowscore;
			}
		}
	}
	printf("%d\n",maxscore>=1000000?-1:maxscore);
	return 0;
}