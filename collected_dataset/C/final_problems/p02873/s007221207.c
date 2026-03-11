#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

int asc(const void *a, const void *b);
int desc(const void *a, const void *b);

int main(void) {

  char *S=(char *)calloc(500000,sizeof(int));
  scanf("%s",S);
  long int l=strlen(S);
  //printf("%d\n",l);

  long int i;
  long int *a=(long int *)calloc(l+1,sizeof(long int));
  for(i=0;i<l;i++){
    //printf("%d %c\n",l,S[i]);
    if(S[i]=='<' && (S[i-1]!='>' || i==0)){
      a[i]=-1;
    }
    else if(S[i]=='>' && (S[i+1]=='<' || i==l-1)){
      a[i+1]=-1;  
  	}
  }
  for(i=0;i<l+1;i++){
    if(S[i]=='<'){
      a[i+1]=a[i]+1;  
  	}
  }
  for(i=l-1;i>=0;i--){
    if(S[i]=='>'){
      a[i]=a[i+1]+1;
    }
  }
  
  
   for(i=0;i<l+1;i++){
   //printf("%d %d\n",i,a[i]);
  	}
  
  long int min=10000000;
  for(i=0;i<l+1;i++){
    //printf("%d %d\n",i,a[i]);
  	if(min>a[i])
    {min=a[i];}
   }
  //printf("%d\n",min);
  long int sum=0;
    for(i=0;i<l;i++){
    if(S[i]=='<' && S[i+1]=='>'){
      	a[i+1]=(a[i]>a[i+2]?a[i]+1:a[i+2]+1);
    }
    }
  
  for(i=0;i<l+1;i++){
   //printf("%d %d\n",i,a[i]-min);
    sum+=a[i]-min;
   }
  printf("%ld\n",sum);
  
}

int asc(const void *a, const void *b) {
  return *(int *)a - *(int *)b;
}
int desc(const void *a, const void *b) {
  return *(int *)b - *(int *)a;
}
