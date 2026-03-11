#include <stdio.h>


int main () {

	


	while(1){
		int x[200]={};
	int y[200]={};

	int input_x = 0;
	int input_y = 0;

	int present_x = 0;
	int present_y = 0;

	int max_x_r=0;
	int max_x_l=0;
	int max_y_u =0;
	int max_y_d=0;

	int result_x=0;
	int result_y=0;

	int der_x=0;
	int der_y=0;

	int left =0;
	int down =1;
	int right = 2;
	int up =3;
	int i=0;

	int dataset = 0;
		scanf("%d",&dataset);

		if ( dataset == 0){
			break;
		}
		//???????????¨???
		x[0] = 0;
		y[0] = 0;

		
		
			for ( i =1; i < dataset; i++){
				scanf("%d %d",&input_x,&input_y);

				//?????¨??????
				present_x = x[input_x];
				present_y = y[input_x];

				if( input_y == 0){
					der_x = -1;
				}
				else if ( input_y == 1){
					der_y = -1;
				}
				else if ( input_y == 2){
					der_x = 1;
				}
				else if ( input_y == 3){
					der_y = 1;
				}

				//??´??°
				x[i] = der_x + present_x;
				y[i] = der_y + present_y;
				der_x = 0;
				der_y = 0;

				//printf("xi:%d yi:%d\n",x[i],y[i] );

				if ( max_x_r < x[i]){
					max_x_r = x[i];
				}
				if(max_x_l > x[i]){
					max_x_l = x[i];
				}
				if ( max_y_u < y[i]){
					max_y_u = y[i];
				}
				if ( max_y_d > y[i]){
					max_y_d = y[i];
				}
			}
			result_x = max_x_r - (max_x_l) + 1;
			result_y = max_y_u - (max_y_d) + 1;
			printf("%d %d\n",result_x,result_y );
			result_x=0;
			result_y=0;

	}

	return 0;
}