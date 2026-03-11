#include<stdio.h>

int arcmp(char a1[],char a2[]);

int main(void){
  int H,W;
  char board[12][13];
  char row[12][26],rowset[12];
  char col[12][26],colset[12];
  int i,j,k;
  int rs=0,cs=0;

  for(i=0;i<12;i++) for(j=0;j<26;j++) row[i][j]=col[i][j]=0;

  scanf("%d%d%*c",&H,&W);
  for(i=0;i<H;i++) scanf("%[a-z]%*c",board[i]);
  for(i=0;i<H;i++){
    for(j=0;j<W;j++){
      row[i][board[i][j]-'a']++;
      col[j][board[i][j]-'a']++;
    }
  }

  for(i=0;i<12;i++) rowset[i]=colset[i]=-1;

  for(i=0;i<H-1;i++){
    if(rowset[i]!=-1) continue;
    for(j=i+1;j<H;j++){
      if(rowset[j]!=-1) continue;
      if(arcmp(row[i],row[j])){
	rowset[i]=j;
	rowset[j]=i;
      }
    }
  }

  for(i=0;i<W-1;i++){
    if(colset[i]!=-1) continue;
    for(j=i+1;j<W;j++){
      if(colset[j]!=-1) continue;
      if(arcmp(col[i],col[j])){
	colset[i]=j;
	colset[j]=i;
      }
    } 
  }

  for(i=0;i<H;i++){
    if(rowset[i]==-1){
      for(j=k=0;j<26;j++) k+=row[i][j]%2;
      if(k!=W%2){
	puts("NO");
	return 0;
      }
      rs++;
    }
  }
  for(i=0;i<W;i++){
    if(colset[i]==-1){
      for(j=k=0;j<26;j++) k+=col[i][j]%2;
      if(k!=H%2){
	puts("NO");
	return 0;
      }
      cs++;
    }
  }

  puts(rs==H%2 && cs==W%2 ? "YES" : "NO");
  return 0;
}

int arcmp(char a1[],char a2[]){
  int i;
  for(i=0;i<26;i++) if(a1[i]!=a2[i]) return 0;
  return 1;
}

  
