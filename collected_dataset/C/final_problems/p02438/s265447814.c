#include <stdio.h>
 
int tail[1000];
int cnt;
int nxt[501010];
int val[501010];
 
int main(){
	int n,q;
	scanf("%d%d",&n,&q);
	for(int i=0;i<n;i++)tail[i]=i,nxt[i]=-1;
	cnt=n;
	while(q--){
		int t;
		scanf("%d",&t);
		if(t==0){
			int k,x;
			scanf("%d%d",&k,&x);
			val[cnt]=x;
			nxt[tail[k]]=cnt;
			tail[k]=cnt;
			nxt[tail[k]]=-1;
			cnt++;
		}else if(t==1){
			int k;
			scanf("%d",&k);
			int idx=nxt[k];
			if(idx==-1)puts("");
			else{
				while(idx!=-1){
					printf("%d%c",val[idx],nxt[idx]==-1?10:32);
					idx=nxt[idx];
				}
			}
		}else{
			int k,kk;
			scanf("%d%d",&k,&kk);
			nxt[tail[kk]]=nxt[k];
			tail[kk]=tail[k];
			tail[k]=k;
			nxt[k]=-1;
		}
	}
}
