#include<stdio.h>
#include<string.h>
int a,b,d[50010],m[50010];
int Y[]={1,2,-1};
int O[]={0,1,-1};
int f(int s,int t){//printf("%d %d %d %d\n",s,t,d[s],m[t]);
  int n,i;
  if(t==b+1&&s==a)return 1;
  if(t>b||s>=a)return 0;
  for(i=0;i<3;i++){
    n=s+Y[i];
    if(n<0||n>a)continue;
    if((d[n]+O[i]+12)%12-m[t]&&n<a)continue;
    if(f(n,t+1))return 1;
  }
  return 0;
}
int main(){
  int n,i,j;
  char s[3],o[][3]={"C","C#","D","D#","E","F","F#","G","G#","A","A#","B"};
  scanf("%d",&n);
  while(n--){
    scanf("%d %d",&a,&b);
    for(i=0;i<a;i++){
      scanf("%s",s);
      for(j=0;strcmp(s,o[j]);j++);
      d[i]=j;
    }
    for(i=0;i<b;i++){
      scanf("%s",s);
      for(j=0;strcmp(s,o[j]);j++);
      m[i]=j;
    }
    d[a]=m[b]=-1;
    printf("%s\n",f(-1,0)?"Yes":"No");
  }
  return 0;
}