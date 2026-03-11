#include<stdio.h>
#define N 1000

int main(void){
  int i,num,j,flag=1;
  char ch[N],cht[N],ansj;

  for(i=0;scanf("%c",&ch[i])==1;i++){}
  num=i;
  for(j=0;j<num;j++){
    cht[i]=ch[i];
  }
  for(j=1;j<=13;j++){
    for(i=0;i<num;i++){
      if('a'<=ch[i] && ch[i]<='z'){
        if('z'<ch[i]){
          cht[i]=ch[i]+j-26;
        }else{
          cht[i]=ch[i]+j;
        }
      }else{
        cht[i]=ch[i];
      }
    }
    for(i=0;i<num;i++){
      if(cht[i]=='t' && cht[i+1]=='h'){
        if(cht[i+2]=='e' ||
            cht[i+2]=='i' && cht[i+3]=='s' ||
            cht[i+2]=='a' && cht[i+3]=='t'){
          ansj=j;
          flag=0;
          break;
        }
      }
    }
    for(i=0;i<num;i++){
      if('a'<=ch[i] && ch[i]<='z'){
        if(ch[i]-j<'a'){
          cht[i]=ch[i]-j+26;
        }else{
          cht[i]=ch[i]-j;
        }
      }else{
        cht[i]=ch[i];
      }
    }
    for(i=0;i<num;i++){
      if(cht[i]=='t' && cht[i+1]=='h'){
        if(cht[i+2]=='e' ||
            cht[i+2]=='i' && cht[i+3]=='s' ||
            cht[i+2]=='a' && cht[i+3]=='t'){
          ansj=-j;
          flag=0;
          break;
        }
      }
    }
    if(flag==0){
      break;
    }
  }
  for(j=0;j<num;j++){
    if('a'<=ch[j] && ch[j]<='z'){
      printf("%c",ch[j]+ansj);
    }else{
      printf("%c",ch[j]);
    }
  }
  return 0;
}