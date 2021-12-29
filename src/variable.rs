// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Eighty.
//
// Copyright (c) 2021 Wei Tang.
//
// Eighty is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Eighty is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Eighty. If not, see <http://www.gnu.org/licenses/>.

use regex::Regex;
use crate::Error;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Variable {
    pub full: String,
    pub name: String,
    pub arguments: Option<String>,
}

pub fn search(content: &str) -> Result<Vec<Variable>, Error> {
    let mut matches = Vec::new();

    let re = Regex::new(r"@@(.+)@@")?;

    for caps in re.captures_iter(content) {
        let full = caps.get(0).ok_or(Error::UnprocessedRegexMatch)?.as_str().to_owned();
        let raw = caps.get(1).ok_or(Error::UnprocessedRegexMatch)?.as_str().to_owned();
        let mut splited = raw.splitn(2, ':');
        let name = splited.next().expect("will return at least one item; qed").to_string();
        let arguments = splited.next().map(|v| v.to_string());
        matches.push(Variable { full, name, arguments });
    }

    Ok(matches)
}
