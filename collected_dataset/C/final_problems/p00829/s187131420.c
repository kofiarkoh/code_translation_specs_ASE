#include<stdio.h>
int main(){
  unsigned int n,i,j,a,b,s,d[10];
  scanf("%d",&n);
  while(n--){
    for(i=0;i<9;i++)scanf("%x",&d[i]);//,printf("%d ",d[i]);printf("\n");
    for(i=a=b=0;i<32;i++){
      for(j=s=0;j<8;j++){
	s+=d[j]%2;
	d[j]/=2;
      }
      if((s+b)%2-d[8]%2){
	a+=1<<i;
	b+=8-s;
      }
      else  b+=s;
      b/=2;
      d[8]/=2;
    }
    printf("%x\n",a);
  }
  return 0;
}