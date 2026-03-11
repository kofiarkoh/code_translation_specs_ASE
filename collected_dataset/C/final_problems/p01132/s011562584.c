#include<stdio.h>
#include<limits.h>

int main(){
  int i,bc[4],price,coins[4],buf[4],bp,end,form[]={10,50,100,500},j,sp;
  int num,c;
  char flg=0;
  do{
    scanf("%d",&price);
    if(!price) break;
    if(flg) printf("\n");
    scanf("%d %d %d %d",coins,coins+1,coins+2,coins+3);
    for(i=0,end=1;i<4;i++){
      buf[i]=bc[i]=0;
      if(coins[i]) end*=coins[i]+1;
    }
    for(end--,i=0,bp=INT_MAX;i<end;i++){
      for(j=0;j<4;j++)
	if(coins[j]!=0&&buf[j]+1<=coins[j]) break;
      buf[j]++;
      for(j--;j>-1;j--) buf[j]=0;
      for(j=0,sp=0;j<4;j++) sp+=buf[j]*form[j];
      if(sp>=price){
	for(num=sp-price,j=3,c=0;j>-1;j--){
	  if(num>=form[j]){
	    c+=num/form[j];
	    num-=form[j]*(num/form[j]);
	  }
	}
	if(c<bp){
	  for(j=0;j<4;j++) bc[j]=buf[j];
	  bp=c;
	}
      }
    }
    for(i=0;i<4;i++)
      if(bc[i]) printf("%d %d\n",form[i],bc[i]);
    flg=1;
  }while(1);
  return 0;
}