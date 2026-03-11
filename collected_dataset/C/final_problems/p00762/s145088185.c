#include<stdio.h>
#include<string.h>
enum D{E,W,S,N} D;
int n,f,t,i,j,r[7],h[105][105],num[105][105];
int dr[7][5]={{},{4,5,3,2},{1,3,6,4},{1,5,6,2},{1,2,6,5},{1,4,6,3},{3,5,4,2}},M[4][2]={{0,1},{-1,0},{0,-1},{1,0}};//SWNE

void rot(int *f,int *t,int d)
{
	int tmp,i;
	for(i=0;i<4;i++)if(dr[*f][i]==*t)break;
	if(d==3){*t=dr[*f][(i-1+4)%4];return;}//E
	if(d==1){*t=dr[*f][(i+1)%4];return;}//W
	if(d==0){tmp=7-*f;*f=*t;*t=tmp;return;}//S
	if(d==2){tmp=7-*t;*t=*f;*f=tmp;return;}//N
	return;
}

void drop(int y,int x,int t,int f)
{
	int mv[2]={0},mm[2]={-1,-1},s,i,j,a,m;
	for(s=0;s<4;s++)if(dr[t][s]==f)break;
	for(i=0;i<4;i++,s=(s+1)%4)
	{
		if(dr[t][s]>3)
		{
			a=dr[t][s];m=i;
			for(j=0;j<2;j++)
				if(mm[j]<dr[t][s])
				{
					a^=mm[j]^=a^=mm[j];
					m^=mv[j]^=m^=mv[j];
				}
		}
	}
	for(i=0;i<2;i++)
	{
		if(h[y+M[mv[i]][1]][x+M[mv[i]][0]]<h[y][x])
		{
			rot(&f,&t,mv[i]);
			drop(y+M[mv[i]][1],x+M[mv[i]][0],t,f);
			return;
		}
	}
	h[y][x]++;
	num[y][x]=t;
	return;
}

int main()
{
	for(;scanf("%d",&n),n;)
	{
		memset(r,0,sizeof(r));
		memset(h,0,sizeof(h));
		memset(num,0,sizeof(num));
		for(;n--;)
		{
			scanf("%d%d",&t,&f);
			drop(51,51,t,f);
		}
		for(i=0;i<105;i++)
			for(j=0;j<105;j++)
				r[num[i][j]]++;
		for(i=1;i<7;i++)
			printf("%d%s",r[i],i==6?"\n":" ");
	}
	return 0;
}