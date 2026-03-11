#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
int d[4][2]={{1, 0},{0, 1},{-1, 0},{0, -1}};
char s[8001];
int r[2], f=0, l, left[8000];

void solve(int x, int y, int dir, int i){
  if(i>l){if(x==r[0] && y==r[1])f=1;  return;}
  if(abs(x-r[0])+abs(y-r[1])>left[i]) return;
  if(s[i]=='F'){
    solve(x+d[dir][0],y+d[dir][1],dir,i+1);
    if(f==1) return;
  }else{
    solve(x,y,(dir+1)%4,i+1);
    if(f==1) return;
    solve(x,y,(dir+3)%4,i+1);
    if(f==1) return;
  }
  return;
}

int main(void){
  int i;
  scanf("%s",s);
  scanf("%d %d",&r[0],&r[1]);
  l = strlen(s);
  if(s[l-1]=='F') left[l-1]=1;
  for(i=0;i<l-1;i++) left[l-2-i] = left[l-1-i]+(s[l-2-i]=='F');
  solve(0,0,0,0);
  if(f==1) printf("Yes\n");
  else printf("No\n");
  return 0;
}
