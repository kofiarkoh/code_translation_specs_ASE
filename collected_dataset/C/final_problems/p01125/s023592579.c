#include<stdio.h>
int main()
{
  int n,x[400],y[400],i,m,j,x1,y1,l,t,x2[400],y2[400];
  char d;
  while(1){
    x1=10;
    y1=10;
    t=0;
    scanf("%d",&n);
    if(n==0)break;
    for(i=0;i<n;i++){
      scanf("%d%d",&x[i],&y[i]);
    }
    scanf("%d",&m);
    for(j=0;j<m;j++){
      scanf("%c",&d);
      scanf("%c",&d);
      scanf("%d",&l);
      if(d=='N'){y1=y1+l;}
      else if(d=='E'){x1=x1+l;}
      else if(d=='S'){y1=y1-l;}
      else if(d=='W'){x1=x1-l;}
    
      for(i=0;i<n;i++){
	if(x2[i]==x1&&y2[i]<=y1||x2[i]<=x1&&y1==y2[i]){
	  t=t;
	} 
        else if(x1==x[i]&&y1>=y[i]){
	  t=t+1;
	  x2[i]=x[i];
	  y2[i]=y[i];}
	else if(x1>=x[1]&&y1==y[i]){
	  t=t=1;
          x2[i]=x[i];
	  y2[i]=y[i];
	}
      
      }
     
    }
    if(t==n){printf("Yes\n");}
    else{printf("No\n");}
    }
  return 0;
  }