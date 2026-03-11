#include <stdio.h>
int main(){

  int n,muki[500],i,j,k,add,mukihow,bash,maxbash;
  char train[1024],name[500],result[500];
  
  scanf("%d",&n);
  
  for(j = 0; j < n; j++){
    
    for(i = 0; i < 1024; i++) train[i]='0';
    for(i = 0; i < 500; i++) name[i]='0';
    for(i = 0; i < 500; i++) muki[i]=0;
    for(i = 0; i < 500; i++) result[i]='0';
    
    
    scanf("%s",train);
    
    add=0;
    for(i = 0; train[i] != '0'; i += 3){
    
    name[add]=train[i];
    add++;
    
  }

    mukihow=add-1;    
    add=0;
    for(i = 1; train[i] != '0'; i += 3){
    
    if(train[i] == '-') muki[add]=1;
    if(train[i] == '<') muki[add]=-1;
    add++;
    
  }

    /*
    printf("%d\n",mukihow);
    

    for(i = 0; i < mukihow+1; i++) printf("%c",name[i]);
    printf("\n");
    
    for(i = 0; i < mukihow; i++) printf("%d",muki[i]);
    printf("\n");
    */    

    bash=0;
    maxbash=0;
    result[0]=name[0];
    
    for(i = 0; i < mukihow; i++){
    bash += muki[i];
    if(maxbash < bash) maxbash=bash;
    if(bash == -1){
    
    for(k = 0; k < maxbash+1; k++) result[maxbash+1-k]=result[maxbash-k];
    maxbash++;
    bash=0;
    
  }

    result[bash]=name[i+1];

  }

    for(i = 0; i < maxbash+1; i++) printf("%c",result[i]);

    printf("\n");
    
    
  }

    return 0;

  }
    
    
    