#include<stdio.h>
#include<string.h>
int main(){
  int n,m,t;
  int i,c;
  char s[12];
  while(1){
    scanf("%d %d",&n,&m);
    if(n==0&&m==0)break;
    scanf("\n");
    t=1;
    c=n;
    int p[1001]={0};
    p[n+1]=-1;
    for(i=1;i<=m;i++){
      if(t==c+1)t=1;
      for(;p[t]==-1;t++);
      if(t==c+1)t=1;
      for(;p[t]==-1;t++);
      fgets(s,10,stdin);
      if(n!=1){
	if(i%3==0){
	  if(i%5==0){
	    if(strcmp(s,"FizzBuzz\n\0")!=0)p[t]=-1;
	  }
	  else{
	    if(strcmp(s,"Fizz\n\0")!=0)p[t]=-1;
	  }
	}
	else{
	  if(i%5==0){
	    if(strcmp(s,"Buzz\n\0")!=0)p[t]=-1;
	  }
	  else{
	    if(!(s[0]==i+'0'&&s[1]==10))p[t]=-1;
	  }
	}
	if(p[t]==-1)n--;
      }
      t++;
    }
    n=0;
    for(i=1;i<=c;i++){
      if(p[i]==0){
	if(n==1)printf(" ");
	printf("%d",i);
	n=1;
      }
    }
    printf("\n");
  }
  return 0;
}