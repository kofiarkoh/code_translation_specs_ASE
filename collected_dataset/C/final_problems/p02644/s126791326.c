#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define ll long long
#define big 123456789

int h,w,k;
char c[1001000];
int came[1001000]={};
int xs,ys,xg,yg;

int xtoc(int y,int x){
  return x+w*(y-1);}


int dy[5]={-1,0,1,0},dx[5]={0,1,0,-1};


int ab(int a){
  if(a>0) return a;
  return -a;}

void dfs(int yfrom,int xfrom){
  //4 directions
  int dirlimit[5];
  for(int dir=0;dir<4;dir++){
    int xtmp = xfrom;
    int ytmp = yfrom;
    for(int i=0;i<=k;i++){
      ytmp+=dy[dir];
      xtmp+=dx[dir];
      if(i==k) break;
      if(ytmp<=0 || ytmp>h || xtmp<=0 || xtmp>w) break;
      if(came[xtoc(ytmp,xtmp)]==-1) break;
      if(came[xtoc(ytmp,xtmp)]<=came[xtoc(yfrom,xfrom)]+1) break;
      came[xtoc(ytmp,xtmp)] = came[xtoc(yfrom,xfrom)]+1;}
    dirlimit[dir] = ab(xtmp-xfrom+ytmp-yfrom)-1;
    //printf("%d %d %d %d\n",yfrom,xfrom,ytmp,xtmp);
    //printf("%d ",dirlimit[dir]);
  }
  //puts("");
  for(int dir=0;dir<4;dir++){
    int xtmp = xfrom;
    int ytmp = yfrom;
    //printf("%d ",dirlimit[dir]);
    for(int i=0;i<dirlimit[dir];i++){
      xtmp += dx[dir];
      ytmp += dy[dir];
      //printf("%d %d\n",ytmp,xtmp);
      dfs(ytmp,xtmp);}
  }
  //puts("");
}
      

int main(){
  scanf("%d%d%d",&h,&w,&k);

  scanf("%d%d%d%d",&xs,&ys,&xg,&yg);
  //int hw = h*w;
  for(int i=0;i<h;i++){
    //printf("%d\n",i);
    char s[123456];
    scanf("%s",s);
    for(int j=1;j<=w;j++){
      int tmp = i*w+j;
      c[tmp] = s[j-1];
    if(c[tmp]=='@') came[tmp]=-1;
    else came[tmp]=big;}}
  //printf("%d\n",hw);

  came[xtoc(xs,ys)] = 0;
  dfs(xs,ys);
  
  if(came[xtoc(xg,yg)]==big) came[xtoc(xg,yg)]=-1;
  /*
  for(int i=1;i<=h;i++){
    for(int j=1;j<=w;j++){
      printf("%d ",came[xtoc(i,j)]);}
    puts("");}*/
  printf("%d\n",came[xtoc(xg,yg)]);
  
  return 0;
}
