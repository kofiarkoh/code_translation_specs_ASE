#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int MAX(int a,int b){return a<b?b:a;}
int n;
char s[200010],t[200010];
int f(char a,char b,int r,int x){
  int i=n-1,l=1,m=r;//printf("%c\n",s[i]);
  if(s[i]==a)l++;
  if(s[i]==b)r--;//printf("%d %d ",l,r);
  if(l>r)return 0;
  while(i--){//printf("%c %c\n",t[i],s[i]);
    if(t[i]==b)l=MAX(1,l-1);
    if(t[i]==a)r=MIN(m,r+1);
    if(s[i]==a)l++;
    if(s[i]==b)r--;//printf("%d %d\n",l,r);
    if(l>r)return 0;
  }
  if(x<l||r<x)return 0;
  return 1;
}  
int main(){
  int h,w,x,y;
  scanf("%d %d %d %d %d %s %s",&h,&w,&n,&y,&x,s,t);
  printf("%s\n",f('U','D',h,y)*f('L','R',w,x)?"YES":"NO");
}
  
