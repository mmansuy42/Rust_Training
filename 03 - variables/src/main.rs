/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mmansuy <mmansuy@student.42perpignan.fr    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/11/08 23:03:12 by mmansuy           #+#    #+#             */
/*   Updated: 2026/11/08 23:03:15 by mmansuy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main()
{
    let x = 5;
    println!("The start value of x is: {x}");
    /* x += 1;
    println!("Did the x value changed ?: {x}"); */
    let x = x + 1;
    println!("Did the x value wont changed ?: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The end value of x is: {x}");
}
