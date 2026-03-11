#include<stdio.h>

int p[5010];
int oya[5010];
int ko[5010][5010],kocnt[5010];
int ans;
void f(int i){
	for(int j=0;j<kocnt[i];j++)if(p[ko[i][j]]==-1){
		p[ko[i][j]]=i;
		p[i]=ko[i][j];
		ans++;
		return;
	}
	if(i==0)return;
	if(p[oya[i]]==-1){
		p[oya[i]]=i;
		p[i]=oya[i];
		ans++;
	}else{
		int t=p[oya[i]];
		p[t]=-1;
		p[oya[i]]=i;
		p[i]=oya[i];
 		if(t<oya[i])f(t);
	}
}

int main(){
	p[0]=-1;
	oya[0]=-1;
	int n;
	scanf("%d",&n);
	for(int i=1;i<=n;i++){
		int t;
		scanf("%d",&t);
		oya[i]=t;
		ko[t][kocnt[t]++]=i;
		f(i);
		printf("%d\n",ans);
	}
}
