
#include <stdio.h> 
#include <stdlib.h> 
#define n_max 500000000
int p_8_8sul[3][3]; 
int k_3_3_template[3][3]; 
int deleta_x_j[4] = {0,1,0,-1}; 
int deleta_y_j[4] = {1,0,-1,0}; 


void ch_em_k(int *x,int *y);
void ch_sw_k(int *a,int *b);
int ch_cl_k();
int ch_df_k(int x,int y,int p,int dep,const int lim);
void o_check1_kaisuu(int );
void o_ch_kaisuu(int );
void o_ch3_kaisuu(int );  

   
int main() { 
  int i,j,cout_ans,x,y,l,n[n_max],h[n_max];
  char p[n_max],liok[n_max];
    int chchch=0;
    int macc;
    int dett;
    int check[5000000000];
    for(int dett=0;dett<9;dett++){
      macc=macc*macc;
    }
    for(dett=0;dett<macc;dett++){
      p[dett]='y';
      liok[dett]='o';
    }
    
    
    for(i=0; i<3; i++){ 
        for(j=0; j<3; j++) scanf("%d",&p_8_8sul[i][j]); 
    }
    /*chchch++;
      printf("%d",chchch);*/
    ch_em_k(&x,&y); 
    for(l=0; l<100; l++){ 
        for(i=0; i<3; i++){ 
            for(j=0; j<3; j++) k_3_3_template[i][j] = p_8_8sul[i][j];
	    /*chchch++;
	      printf("%d",chchch);*/
        } 
           
        cout_ans = ch_df_k(x,y,-1,0,l); 
        if(cout_ans!=-1){ 
            printf("%d\n",cout_ans); 
            break; 
        } 
           
        for(i=0; i<3; i++){ 
            for(j=0; j<3; j++) p_8_8sul[i][j] = k_3_3_template[i][j]; 
        } 
        /*chchch++;
	  printf("%d",chchch);*/   
        cout_ans = -1; 
        if(l==99) printf("%d\n",cout_ans); 
    } 
       
    return 0; 
}




void ch_em_k(int *x,int *y) { 
  int i,j; 
  for(i=0; i<3; i++) { 
    for(j=0; j<3; j++) { 
      if(!p_8_8sul[i][j]) { 
*x = j; 
*y = i; 
return; 
      } 
    } 
  } 
} 

void o_check_kaisuu(int w){
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





void ch_sw_k(int *a,int *b){ 
  int k_3_3_template; 
  k_3_3_template = *a; 
  *a = *b; 
  *b = k_3_3_template; 
} 
   
int ch_cl_k(){ 
  int i,a,a_b_p_p=0;  
  for(i=0; i<9; i++){ 
    a = p_8_8sul[i/3][i%3]; 
    if(a){ 
      a--; 
      a_b_p_p += abs(i/3 - a/3) + abs(i%3 - a%3); 
    } 
  }     
  return a_b_p_p; 
} 

void o_ch_kaisuu(int w){
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





int ch_df_k(int x,int y,int p,int dep,const int lim){ 
    int i,h,nx,ny,res;    
    h = ch_cl_k(); 
    if(h==0) return dep; 
    if(dep+h>lim) return -1;  
    for(i=0; i<4; i++){ 
        nx = x+deleta_x_j[i]; 
        ny = y+deleta_y_j[i];       
        if(~p && i==(p+2)%4) continue; 
        if(!(0<=nx && nx<3 && 0<=ny && ny<3)) continue;     
        ch_sw_k(&p_8_8sul[y][x],&p_8_8sul[ny][nx]); 
        res = ch_df_k(nx,ny,i,dep+1,lim); 
        if(~res) return res;        
        ch_sw_k(&p_8_8sul[y][x],&p_8_8sul[ny][nx]); 
    }    
    return -1; 
}





void o_ch3_kaisuu(int w){
  int i=0;
  int m=7;
  int check[5000000000];
  for(i=0;i<7;i++){
    m=m*m;
  }
  for(i=0;i<m;i++){
    check[i]=m;
  }
}


/*#include <stdio.h>
#include <stdbool.h>


#define Niii 8
#define freeqqq -1
#define not_freewww 1

void inter(void);
void board(void);
void recursive(int );

int row[Niii],col[Niii],dpos[2*Niii-1],dneg[2*Niii-1];

bool X[Niii][Niii];



int main()
{
  inter();

  for(int i=0;i<Niii;i++){
    for(int j=0;j<Niii;j++){
      X[i][j]=false;

    }
  }

  int k;
  scanf("%d",&k);
  for(int i=0;i<k;i++){
    int r,c;
    scanf("%d%d",&r,&c);
    X[r][c]=true;
  }

  recursive(0);
  return 0;
}



void inter(){
  int i;
  for(i=0;i<Niii;i++){
    row[i]=freeqqq;
    col[i]=freeqqq;
  }
  for(i=0;i<2*Niii-1;i++){
    dpos[i]=freeqqq;
    dneg[i]=freeqqq;
  }
}


void board(){
  int i,j;
  for(i=0;i<Niii;i++){
    for(j=0;j<Niii;j++){
      if(X[i][j]){
	if(row[i]!=j)return;
      }
    }
  }
  for(i=0;i<Niii;i++){
    for(j=0;j<Niii;j++){
      if(row[i]==j){
	printf("Q");
      }
      else{
	printf(".");
      }
    }
    printf("\n");
  }
}


void recursive(int i){
  if(i==Niii){
    board();return;
  }
  for(int j=0;j<Niii;j++){
    if(not_freewww==col[j] || not_freewww==dpos[i+j] || not_freewww==dneg[i-j+Niii-1])continue;
    row[i]=j;
    col[j]=not_freewww;
    dpos[i+j]=not_freewww;
    dneg[i-j+Niii-1]=not_freewww;
    recursive(i+1);
    row[i]=col[j]=dpos[i+j]=dneg[i-j+Niii-1]=freeqqq;
  }
}
*/


