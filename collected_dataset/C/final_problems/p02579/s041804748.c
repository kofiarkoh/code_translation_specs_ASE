#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define rep(i,N) for(int i=0;i<(int)N;i++)
#pragma GCC optimize("O3")
#pragma GCC target("sse,sse2,sse3,ssse3,sse4,popcnt,abm,mmx,avx,tune=native")
static inline void PUT(char c)
{
  static char buf[1<<15],*ptr=buf;
  if(ptr==buf+strlen(buf)||c==0){fwrite(buf,1,ptr-buf,stdout),ptr=buf;}*ptr++=c;
}
static inline int IN(void)
{
  int x=0,f=0,c=getchar();while(c<48||c>57){f^=c==45,c=getchar();}
  while(c>47&&c<58){x=x*10+c-48,c=getchar();}return f?-x:x;
}
static inline void OUT(int a)
{
  if(a<0)PUT('-'),a=-a;
  int d[40],i=0;do{d[i++]=a%10;}while(a/=10);
  while(i--){PUT(d[i]+48);}PUT('\n');
}
const int qsz=1e6;int qLen,qHead,Q[1000000];
static inline void PUSH(int n){Q[(qHead+qLen)%qsz]=n;qLen++;}
static inline int POP(void){int q=Q[qHead];qLen--;qHead=(qHead+1)%qsz;return q;}
int H,W,grid[1000][1000];
const int dx[24]={0,1,0,-1,0,0,1,1,1,1,-1,-1,-1,-1,2,2,2,2,2,-2,-2,-2,-2,-2};
const int dy[24]={1,0,-1,0,2,-2,1,2,-1,-2,1,2,-1,-2,0,1,2,-1,-2,0,1,2,-1,-2};
static inline int f(int x,int y){return W*x+y;}
static inline void DFS(int x,int y)
{
  if(x>=0&&x<H&&y>=0&&y<W&&!grid[x][y])
  {
    grid[x][y]=1;PUSH(f(x,y));rep(i,4){DFS(x+dx[i],y+dy[i]);}
  }
}
int main(void)
{
  int C[2],D[2],step=0;H=IN(),W=IN(),C[0]=IN()-1,C[1]=IN()-1,D[0]=IN()-1,D[1]=IN()-1;
  rep(i,H){char str[1001];if(!scanf("%s",str)){return -1;}rep(j,W){grid[i][j]=(str[j]=='#');}}
  PUSH(f(C[0],C[1]));
  while(qLen)
  {
    int size=qLen;
    while(size--){int x=POP();DFS(x/W,x%W);}
    size=qLen;
    while(size--)
    {
      int z=POP(),x=z/W,y=z%W;
      if(x==D[0]&&y==D[1]){return OUT(step),0;}
      rep(i,20)
      {
        int nx=x+dx[4+i],ny=y+dy[4+i];
        if(nx>=0&&nx<H&&ny>=0&&ny<W&&!grid[nx][ny]){PUSH(f(nx,ny));}
      }
    }
    step++;
  }
  return puts("-1"),0;
}