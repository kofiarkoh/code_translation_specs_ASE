//2020.1.22
//s1270188 xxxmk2
//icpc_2019 pre2

#include<stdio.h>
#define N 50
#define LONG 1000

//global
char data[N][N+1], input[LONG+1];
int x, y, h, w;

//prototype
int find(char,int,int);

int main(){
  int i, sum;

  while(1){
    //input
    scanf("%d %d",&h,&w);
    if(h==0 && w==0) break;
    for(i=0;i<h;i++) scanf(" %s",data[i]);
    scanf(" %s",input);

    //count
    for(i=sum=x=y=0;input[i]!='\0';i++) sum+=find(input[i],x,y);
    
    //output
    printf("%d\n",sum);
  }
  
  return 0;
}



int find(char a,int xx,int yy){
  int i, j, cnt=0;

  for(i=0;i<h;i++){
    for(j=0;j<w;j++) if(data[i][j]==a) break;
    if(j<w) break;
  }

  if(xx<i) cnt+=i-xx;
  else cnt+=xx-i;

  if(yy<j) cnt+=j-yy;
  else cnt+=yy-j;

  x=i;
  y=j;
  
  return cnt+1;
}

