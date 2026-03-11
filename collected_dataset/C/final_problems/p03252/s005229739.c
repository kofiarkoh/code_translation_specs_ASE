#include <stdio.h>
#include <string.h>

void checkint(int N,int*Sn,int*Tn){
  int i;
  int flag=0;
  for(i=0;i<N;i++){
    if(Sn[i]!=Tn[i]){
      flag=flag+1;
    }
  }
  if(flag==0){
    printf("Yes");
  }else{
    printf("No");
  }
}

void sortchar(int N,char*S,int*Sn){
  int i;
  int count=1;
  int R[100];
  for(i=0;i<100;i++){
    R[i]=0;
  }
  
  for(i=0;i<N;i++){
    if(R[S[i]-'a']==0){
      Sn[i]=count;
      R[S[i]-'a']=count;
      count=count+1;
    }else{
      Sn[i]=R[S[i]-'a'];
    }
  }
}

int main(){
  char S[200000];
  char T[200000];
  scanf("%s",S);
  scanf("%s",T);
  int N;
  N=strlen(S);
  
  int Sn[N];
  int Tn[N];
  sortchar(N,S,Sn);
  sortchar(N,T,Tn);
  checkint(N,Sn,Tn);
  return 0;
}

  
