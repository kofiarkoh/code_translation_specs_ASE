// AOJ Volume 0 Problem 0045

#include <stdio.h>

int main(void)
{
    int price;
    int no;
    int sum_price;
	int sum_no;
	int no_data;

    sum_no = sum_price = no_data = 0;
    
    while (scanf("%d,%d", &price, &no) != EOF){
		sum_price += (price * no);
		sum_no += no;
		no_data++;
    }

	printf("%d\n%.0f\n", sum_price, (float)sum_no / (float)no_data);
	
	return (0);
}