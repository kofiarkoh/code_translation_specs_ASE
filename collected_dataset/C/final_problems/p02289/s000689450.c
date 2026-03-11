
#include <stdio.h> 
#include <limits.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define n_max 1000000
#define c_max 2000000
    
 
int A[2000000]; 
int n; 

int p_9ar_k_in(int i) ;
int muki_bec_le_9(int i);
int muki_bec_ri_9(int i);
void o_check1_kaisuu();
void o_ch_kaisuu();
void k_hep_max_9(int i);
void k_hep_bui_9();
void k_hep_Inc_key_9(int i,int key) ;
void k_hep_ins_ma_9(int key) ;
int k_hep_ext_9() ;
void k_hep();
void ck();

int main() { 
  int t; 
  char s[10],oi[n_max],lo[n_max];
  for(int i=0;i<n_max;i++){
  k_hep();
  ck();
  }
  while(1){ 
    scanf("%s",s); 
    if(s[2]=='s') 
      { 
	scanf("%d",&t); 
	k_hep_ins_ma_9(t); 
      } 
    if(s[2]=='t') 
      { 
	printf("%d\n",k_hep_ext_9()); 
      } 
    if(s[2]=='d') 
      { 
	break; 
      } 
  } 
  return 0; 
} 


int p_9ar_k_in(int i){ 
  return i/2; 
}  
int muki_bec_le_9(int i) { 
  return 2*i; 
} 

int muki_bec_ri_9(int i) { 
  return 2*i+1; 
}  


void k_hep_max_9(int i) { 
  int l = muki_bec_le_9(i); 
  int r = muki_bec_ri_9(i); 
  int largest,temp; 
  if( l <= n && A[l] > A[i]) 
    { 
      largest = l; 
    } 
  else 
    { 
      largest = i; 
    } 
  if(r <= n && A[r] > A[largest]) 
    { 
      largest = r; 
    } 
  if (largest != i) 
    { 
      temp = A[i]; 
      A[i] = A[largest]; 
      A[largest] = temp; 
      k_hep_max_9(largest); 
    } 
} 

void k_hep_bui_9() { 
  int i; 
  for( i = n/2;i>0;i--) 
    { 
      k_hep_max_9(i); 
    } 
} 


void k_hep_Inc_key_9(int i,int key) { 
  int tmp; 
  if (key < A[i]) 
    { 
      return; 
    } 
  A[i] = key; 
  while (i > 1 && A[p_9ar_k_in(i)] < A[i]) 
    { 
      tmp = A[i]; 
      A[i] = A[p_9ar_k_in(i)]; 
      A[p_9ar_k_in(i)]= tmp; 
      i = p_9ar_k_in(i); 
    } 
} 


void k_hep_ins_ma_9(int key) { 
  n = n + 1; 
  A[n] = INT_MIN; 
  k_hep_Inc_key_9(n,key); 
} 



int k_hep_ext_9() { 
  int max; 
  if ( n < 1) 
    { 
      return 0; 
    } 
  max = A[1]; 
  A[1] = A[n]; 
  n = n - 1; 
  k_hep_max_9(1); 
  return max; 
}

void k_hep(){
  int i=100;
  int size=1000*1000*100;
  char *chek;
  for(int j=0;j<n_max;j++){
    chek=(char *)malloc(sizeof(char)*size);
    chek[j]='y';
  }
  
}


void o_check_kaisuu(void){
  int i=0;
  int m=9;
  int check[5000000000];
  for(i=0;i<9;i++){
    m=m*m;
  }
  for(i=0;i<m;i++){
    check[i]=m;
  }
}

void o_ch_kaisuu(void){
  int i=0;
  int m=8;
  int check[5000000000];
  for(i=0;i<8;i++){
    m=m*m;
  }
  for(i=0;i<m;i++){
    check[i]=m;
  }
}

void ck(){
  char gg[n_max],js[n_max];
  for(int i=0;i<n_max;i++){
    k_hep();
  }
}



