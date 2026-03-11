#include<stdio.h>
#include<string.h>

int main(void)
{
  int n,a,s,g;
  int i,j,c,d;
  int x[400],y[400];
  char lag[400][7];
  char min[40][7];
  int tmp,road[40];
  char ans[300];

  while(1){
    scanf(" %d %d %d %d ",&n,&a,&s,&g);
    if(n==0 && a==0 && s==0 && g==0)break;
    for(i=0;i<a;i++)scanf( "%d %d %s ",&x[i],&y[i],lag[i]);
    for(i=0;i<n;i++){strcpy(min[i],"zzzzzz");}
    c=0;d=0;road[0]=s;

    for(i=0;i<40;i++){
    	tmp = road[i];
      for(j=0;j<a;j++){
				if(x[j]==road[i]){
	  			if(strcmp(lag[j],min[i])<=0){
	    			strcpy(min[i],lag[j]);
	    			tmp=y[j];
	  			}
				}
      }
      road[i+1]=tmp;
      if(tmp==g)c=1;
      for(j=0;j<i+1;j++){
				if(tmp==road[j]){
	 				c=1;
	  			d=1;
				}
      }
      if(c==1)break;

    }

    if(d==1){
      printf("NO\n");
    }else{
      strcpy(ans,min[0]);
      for(j=1;j<i+1;j++)strcat(ans,min[j]);
      printf("%s\n",ans);
    }
  }
  return 0;
}