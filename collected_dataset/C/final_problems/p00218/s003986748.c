#include <stdio.h>
typedef struct {
  int pm;
  int pe;
  int pj;
  int rank;
}set;

  int main(){
  set student[10000];
  int i ,n;
  while( scanf( "%d" ,&n ) != EOF ){
    if( n == 0 ){
      break;
    }

    for( i=0 ; i<n ; i++ ){
      scanf( "%d %d %d" ,&student[i].pm ,&student[i].pe ,&student[i].pj );
      
      if( student[i].pm == 100 || student[i].pe == 100 || student[i].pj == 100 ){
	student[i].rank = 1;
      }  /* 100点があったらクラスはA */
      
      else if( (student[i].pm + student[i].pe) / 2 >= 90 ){
	student[i].rank = 1;
      }  /* 数学と英語の平均点が90を超えたらクラスはA */
      
      else if( ( student[i].pm + student[i].pe + student[i].pj ) / 3 >= 80 ){
	student[i].rank = 1;
      }  /* ３教科の平均が80を超えたらクラスはA */
      
      else if( ( student[i].pm + student[i].pe + student[i].pj ) / 3 >= 70 ){
	student[i].rank = 2;
      }  /* ３教科の平均が70を超えたらクラスはB */
      
      else if(  ( student[i].pm + student[i].pe + student[i].pj ) / 3 >= 50 && student[i].pm >= 80 || student[i].pe >= 80 ){
	student[i].rank = 2;
      }  /* 3教科の平均が50を越え、数学か英語が80以上ならクラスはB */
      
      else{
	student[i].rank = 3;
      }  /* その他の場合のクラスはC */
      
    }
    
    for( i=0 ; i<n ; i++ ){
      
      if( student[i].rank == 1 ){
	printf( "A\n" );
      }
      if( student[i].rank == 2 ){
	printf( "B\n" );
      }
      
      if( student[i].rank == 3 ){
	printf( "C\n" );
      }
      
    }

  }

  return 0;
}