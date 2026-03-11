#include<stdio.h>

int main(){
  int i, j, l, m, bfmax, afmax, tmp, kari, n, k, c[10];

  while(1){
    scanf("%d%d", &n, &k);
    if(n==0 && k==0){
      break;
    }
    for(i=0; i<n; i++){
      scanf("%d", &c[i]);
    }

    bfmax = afmax = 1;
    for(i=0; i<n-k+1; i++){
      kari=1;
      for(j=i; j<k+i; j++){
	kari*=c[j];
      }
      if(kari>bfmax){
	bfmax=kari;
      }
    }
    
    for(i=0; i<n-1; i++){
      for(j=i+1; j<n; j++){
	tmp=c[i];
	c[i]=c[j];
	c[j]=tmp;
	for(l=0; l<n-k+1; l++){
	  kari=1;
	  for(m=l; m<k+l; m++){
	    kari*=c[m];
	  }
	  if(kari>afmax){
	    afmax=kari;
	  }
	}
	tmp=c[i];
	c[i]=c[j];
	c[j]=tmp;
	
      }
    }

    if(afmax>=bfmax){
      printf("%d\n",afmax-bfmax);
    }else{
      printf("NO GAME\n");
    }
  }

  return 0;
}