#include<stdio.h>
int main(void){
  int i,j,n,c=0,b=0,s=0;
  char in[100];
  int ten[100]={0};
  
  scanf("%d",&n);
  while(c!=n){
    scanf("%s",in);
    switch(in[1]-'A'){
    case 8:b++;break;  //i hit
    case 20:s++;break;  //u out   
    case 14:ten[c]+=b+1;b=0;break;//o homerun
    }
    if(b==4){ten[c]++;b=3;}
    if(s==3){c++;b=0;s=0;}
  }

  for(i=0;i<n;i++){
    printf("%d\n",ten[i]);
  }
  return 0;

}