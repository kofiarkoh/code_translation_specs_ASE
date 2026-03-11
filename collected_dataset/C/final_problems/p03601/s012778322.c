#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int main(){
  int a,b,c,d,e,f,i,j,s[10010],w[10010],x=1,y=0;
  scanf("%d %d %d %d %d %d",&a,&b,&c,&d,&e,&f);
  for(i=0;i<f;i++)s[i+1]=w[i+1]=-1;
  s[0]=w[0]=0;
  for(i=0;i*100<=f;i++){
    // printf("%d\n",w[i]);
    w[i+1]=MAX(w[i+1],w[i]);
    w[i+a]=MAX(w[i+a],w[i]+a);
    w[i+b]=MAX(w[i+b],w[i]+b);
  }
  for(i=0;i*100<f*e;i++){
    //printf("%d ",s[i]);
    s[i+1]=MAX(s[i+1],s[i]);
    s[i+c]=MAX(s[i+c],s[i]+c);
    s[i+d]=MAX(s[i+d],s[i]+d);
  }//printf("\n");
  for(i=0;i*100<f;i++){
    for(j=0;j*100<f*e;j++){
      if(s[j]>w[i]*e||w[i]*100+s[j]>f)continue;
      if((w[i]*100+s[j])*y<(x+y)*s[j]){
	x=w[i]*100;
	y=s[j];
      }
    }
  }
  printf("%d %d\n",x+y,y);
  return 0;
}
