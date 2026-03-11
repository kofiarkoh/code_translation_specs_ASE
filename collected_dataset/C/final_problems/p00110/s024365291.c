#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
	char str[127] = {}, tmp[127] = {}, num1[127] = {}, num2[127] = {}, ans[127] = {}, tpans[127] = {};
	char *tp;
	int i, j = 0, k = 0, sum = 0, len = 0, cnt = 0;
	int flag = 0, keta1, keta2, ketaans, min = 0, carry = 0, tpas = 0;

	while ( fgets( str, 126, stdin ) != NULL ) {
		flag = -1;

		// 文字列中のXの値を0~9まで変化さ，左辺と右辺が同一となる場合を探す
		// 同一となる場合はflagの値にその数字を入れる
		// 同一とならない場合はflag に-1の値を入れる
		for ( i = 0 ; i < 10 ; i++ ) {
			keta1 = keta2 = ketaans = 0;	
			for ( j = 0 ; j < 126 ; j++ ) {
				tmp[j] = ans[j] = num1[j] = num2[j] = tpans[j] = '0';
			}

			strcpy( tmp, str );
			tp = tmp;
			j = 0;
			// 文字列中のXの値をiに変更する
			while ( *tp != '\0' ) {
				if ( *tp == 'X' ) {
					*tp = '0' + i;			
				}
				tp++;
			}

			len = strlen( tmp )-1; // 最後の１文字分は'\0'なのでカウントから除外する
			tp = tmp+len-1;
			while ( *tp != '=' ) {
				ans[ketaans] = *tp;
				ketaans++;
				len--;
				tp--;
			}
			//printf( "%s=", ans );
			
			// 現在のtpの値は=のため, 位置を進める
			tp--;
			while ( *tp != '+' ) {
				num1[keta1] = *tp;
				keta1++;
				len--;
				tp--;
			}
			//printf( "%s+", num1 );
			
			// 現在のtpの値は+のため, 位置を進める
			tp--;
			while ( len > -1 ) {
				num2[keta2] = *tp;
				keta2++;
				len--;
				tp--;
			} 
			//printf( "%s\n", num2 );

			if ( keta1 > keta2 ) {
				min = keta1;
			} else {
				min = keta2;
			}
		
			carry = 0;
			for ( j = 0 ; j < min ; j++ ) {
				// num1, num2の値が数値の場合にのみ計算を行う
				if ( num1[j] >= '0' && num1[j] <= '9' && num2[j] >= '0' && num2[j] <= '9' ) {
					tpas = carry + num1[j] - '0' + num2[j] - '0';
					// printf( "%d\n", tpas );
					if ( tpas > 9 ) {
						carry = 1;
						tpans[j] = tpas%10 + '0';
					} else {
						carry = 0;
						tpans[j] = tpas + '0';
					}
				}
			}

			if ( carry == 1 ) {
				tpans[min] = '1';
			}

			cnt = 0;
			for ( j = 0 ; j < ketaans ; j++ ) {
				if ( ans[j] == tpans[j] ) {
					cnt++;
				}
			}

			if ( cnt == ketaans ) {
				flag = i;
				break;
			}
		}

		if ( flag == -1 ) {
			printf( "NA\n" );
		} else {
			printf( "%d\n", flag );
		}
	}
	return 0;
}

