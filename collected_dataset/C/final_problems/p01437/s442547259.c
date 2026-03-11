#include<stdio.h>
#include<string.h>
char M[105][105],c,D[5]="NESW";
int H,W,sh,sw,i,j,t[105][105][4],mv[4]={-1,0,1,0},d,nh,nw;
long long int L;
int out(int a,int b){return 0>a||a>=b;}
int main()
{
	for(;scanf("%d%d%lld\n",&H,&W,&L),H;)
	{
		memset(t,-1,sizeof(t));
		memset(M,0,sizeof(M));
		for(i=0;i<H;i++)
			for(j=0;j<=W;j++)
			{
				c=getchar();
				if('A'<c&&c<'Z')
				{
					d=c=='N'?0:c=='E'?1:c=='S'?2:3;
					sh=i;sw=j;M[i][j]='.';
				}
				else M[i][j]=c;
			}
		for(i=0;i<L;)
		{
			nh=sh+mv[d];nw=sw+mv[(d+1)%4];
			if(out(nh,H)||out(nw,W)||M[nh][nw]=='#')
			{
				d=(d+1)%4;
				continue;
			}
			if(t[sh][sw][d]!=-1)
			{
				L=(L-i)%(i-t[sh][sw][d]);
				i=0;
				memset(t,-1,sizeof(t));
				continue;
			}
			t[sh][sw][d]=i++;
			sh=nh;sw=nw;
		}
		printf("%d %d %c\n",sh+1,sw+1,D[d]);
	}
	return 0;
}