#include<stdio.h>
int i,x,y,j,n,m;
long long p,q,min,sum,s;
int map[100][100];
long long cost[100][100];
long long time[100][100];
long long cur[100],tim[100];
long long f[100][3000];
void search(int k,long long money,long long t){
	if (f[k][money]>t)
	  f[k][money]=t;else
	  return;
	int i;
	for (i=1;i<=n;i++)
	  if (map[k][i]==1&&money-cost[k][i]>=0){
	  	search(i,money-cost[k][i],t+time[k][i]);
	  }
	for (i=1;i<=50000;i++){
		if (money+i*cur[k]>=sum){
			search(k,sum,t+i*tim[k]);
			break;
		}else{
			search(k,money+i*cur[k],t+i*tim[k]);
		}
	}
}
int main(){
	scanf("%d %d %lld",&n,&m,&s);
	for (i=1;i<=m;i++){
	  scanf("%d %d %lld %lld",&x,&y,&p,&q);
	  map[x][y]=1;
	  map[y][x]=1;
	  cost[x][y]=p;
	  cost[y][x]=p;
	  time[x][y]=q;
	  time[y][x]=q;
	  sum=sum+p;	
	}
	for (i=1;i<=n;i++){
		scanf("%lld %lld",&cur[i],&tim[i]);
	}
	for (i=1;i<=n;i++)
	  for (j=0;j<=2500;j++){
	  f[i][j]=2147483647;	
	}
	if (s<sum)
	search(1,s,0);
	else 
	search(1,sum,0);
	for (i=2;i<=n;i++){
		min=2147483647;
		for (j=0;j<=sum;j++)
		  if (f[i][j]<min)
		    min=f[i][j];
	    printf("%lld\n",min);
	}
} 