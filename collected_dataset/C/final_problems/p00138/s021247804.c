#include<stdio.h>
int main(){
  int i,j,k,b,m1=100,m2=100;
  double a,mt1=100,mt2=100;
  for(k=0;k<3;k++){
    int id,n[5]={100,100,100,100,100};
    double t,r[5]={100,100,100,100,100};
    for(i=0;i<8;i++){
      scanf("%d %lf",&id,&t);
      for(j=3;j>=0;j--){
	if(t<r[j]){
	  r[j+1]=r[j];
	  r[j]=t;
	  n[j+1]=n[j];
	  n[j]=id;
	}
      }
    }
    if(mt2>r[2]){
      mt2=r[2];
      m2=n[2];
      if(mt1>mt2){
	a=mt1;
	mt1=mt2;
	mt2=a;
	b=m1;
	m1=m2;
	m2=b;
	if(mt2>r[3]){
	  mt2=r[3];
	  m2=n[3];
	}	
      }
    }
    printf("%d %.2f\n",n[0],r[0]);
    printf("%d %.2f\n",n[1],r[1]);
  }
  printf("%d %.2f\n",m1,mt1);
  printf("%d %.2f\n",m2,mt2);
  return 0;
}