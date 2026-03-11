#include<stdio.h>
int main(){
  int n,m,a;
  double s,p,sp,max=0,e=0.0000001;
  scanf("%d",&n);
  while(n--){
    scanf("%*d %d",&m);
    for(s=sp=0;m--;s+=a*p){
      scanf("%d %lf",&a,&p);
      sp+=p;
    }
    if(max+e<s/sp)max=s/sp;
  }
  scanf("%*d %d",&m);
  for(s=sp=0;m--;s+=a*p){
    scanf("%d %lf",&a,&p);
    sp+=p;
  }//printf("%f %f\n",s/sp,max);
  printf("%s\n",s/sp+e<max?"YES":"NO");
  return 0;
}